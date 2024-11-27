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

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting application!");
    let peripherals = Peripherals::take().unwrap();

    // MPU
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio25.into();
    let scl = peripherals.pins.gpio21.into();
    let baudrate = KiloHertz(400).into();

    let mut mpu = MPU6886::new(i2c, sda, scl, baudrate);
    mpu.init();

    // LED Matrix
    let mut matrix = RGBMatrix::new(5, 5, peripherals.pins.gpio27, peripherals.rmt.channel0);
    let player_color = RGB8::new(0, 0, 100);

    // Minigame
    let mut game = MiniGame::new(3.0);

    let delay: Delay = Default::default();
    let mut last_frame = Instant::now();
    let frame_target = std::time::Duration::from_micros(16667); // ca. 60fps

    loop {
        let frame_start = Instant::now();
        let delta = frame_start.duration_since(last_frame).as_secs_f32();

        matrix.clear();
        matrix.set_xy_rgb(game.curr_pos.x.into(), game.curr_pos.y.into(), player_color);

        let acc = mpu.get_acc_data();
        let pitch = acc.y.atan2(acc.x.powi(2) + acc.z.powi(2)) * 180.0 / PI;
        let roll = acc.x.atan2(acc.y.powi(2) + acc.z.powi(2)) * 180.0 / PI;

        let speed_multiplier = 10.0;
        game.update_position_with_delta(pitch, roll, delta * speed_multiplier);

        // Maintain framerate
        let frame_time = frame_start.elapsed();
        if frame_time < frame_target {
            delay.delay_ms(((frame_target - frame_time).as_micros() / 1000) as u32);
        }

        last_frame = frame_start;
    }
}