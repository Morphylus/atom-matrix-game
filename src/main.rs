use std::f32::consts::PI;
use std::time::Instant;

use esp_idf_svc::hal::units::KiloHertz;
use esp_idf_svc::hal::{delay::Delay, prelude::Peripherals};

mod matrix;
mod minigame;
mod mpu6886;
use matrix::{RGBMatrix, RGB8};
use minigame::MiniGame;
use mpu6886::MPU6886;

type GenError = Box<dyn std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting application!");
    let peripherals = Peripherals::take().expect("Initializing peripherals failed");

    // MPU
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio25.into();
    let scl = peripherals.pins.gpio21.into();
    let baudrate = KiloHertz(400).into();

    let mut mpu = MPU6886::new(i2c, sda, scl, baudrate)
        .expect("Failed to initialize MPU driver")
        .init()
        .expect("MPU configuration failed");

    // LED Matrix
    let mut matrix = RGBMatrix::new(5, 5, peripherals.pins.gpio27, peripherals.rmt.channel0)
        .expect("Unable to initialize LED Matrix driver");
    let player_color = RGB8::new(0, 0, 100);

    // Minigame
    let mut game = MiniGame::new(3.0);

    let delay: Delay = Default::default();
    let mut last_frame = Instant::now();
    let frame_target = std::time::Duration::from_micros(16667); // ca. 60fps

    loop {
        let frame_start = Instant::now();
        let delta = frame_start.duration_since(last_frame).as_secs_f32();

        matrix.clear().expect("Clear matrix failed");
        matrix
            .set_xy_rgb(game.curr_pos.x.into(), game.curr_pos.y.into(), player_color)
            .expect("Setting pixel color failed");

        let acc = match mpu.get_acc_data() {
            Ok(data) => data,
            Err(_) => continue,
        };

        let pitch = acc.y.atan2(acc.x.powi(2) + acc.z.powi(2)) * 180.0 / PI;
        let roll = acc.x.atan2(acc.y.powi(2) + acc.z.powi(2)) * 180.0 / PI;

        let tilt_magnitude = (pitch.powi(2) + roll.powi(2)).sqrt();
        let normalized_tilt = (tilt_magnitude / 90.0).clamp(0.0, 1.0);
        let speed_multiplier = 1.0 + (1.0 - (1.0 - normalized_tilt).powf(4.0)) * 15.0;
        game.update_position_with_delta(pitch, roll, delta * speed_multiplier);

        // Maintain framerate
        let frame_time = frame_start.elapsed();
        if frame_time < frame_target {
            delay.delay_ms(((frame_target - frame_time).as_micros() / 1000) as u32);
        }

        last_frame = frame_start;
    }
}
