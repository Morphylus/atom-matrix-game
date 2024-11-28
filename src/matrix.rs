use core::time::Duration;
use esp_idf_svc::hal::{
    gpio::OutputPin,
    peripheral::Peripheral,
    rmt::{config::TransmitConfig, PinState, Pulse, RmtChannel, TxRmtDriver, VariableLengthSignal},
};

use crate::GenResult;

pub struct RGBMatrix<'a> {
    pub led_array: Vec<RGB8>,
    pub tx_driver: TxRmtDriver<'a>,
    x_dim: usize,
    y_dim: usize,
}

impl<'d> RGBMatrix<'d> {
    pub fn new(
        x_dim: usize,
        y_dim: usize,
        led: impl Peripheral<P = impl OutputPin> + 'd,
        channel: impl Peripheral<P = impl RmtChannel> + 'd,
    ) -> GenResult<Self> {
        let config = TransmitConfig::new().clock_divider(2);
        let tx_driver = TxRmtDriver::new(channel, led, &config)?;
        Ok(RGBMatrix {
            led_array: vec![RGB8::new(0, 0, 0); x_dim * y_dim],
            tx_driver,
            x_dim,
            y_dim,
        })
    }

    pub fn clear(&mut self) -> GenResult<()> {
        for color in self.led_array.iter_mut() {
            *color = RGB8::new(0, 0, 0);
        }
        self.refresh_leds()?;
        Ok(())
    }

    pub fn _set_rgb(&mut self, pixel: usize, rgb: RGB8) -> GenResult<()> {
        match self.led_array.get_mut(pixel) {
            Some(curr_color) => {
                *curr_color = rgb;
                self.refresh_leds()?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn set_xy_rgb(&mut self, x: usize, y: usize, rgb: RGB8) -> GenResult<()> {
        match self.led_array.get_mut(y * self.y_dim + x) {
            Some(curr_color) => {
                *curr_color = rgb;
                self.refresh_leds()?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn refresh_leds(&mut self) -> GenResult<()> {
        let matrix_size = self.x_dim * self.y_dim;
        let mut signal = VariableLengthSignal::with_capacity(24 * matrix_size);
        let ticks_hz = self.tx_driver.counter_clock()?;

        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &350.ns())?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &800.ns())?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &700.ns())?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &600.ns())?;

        for i in 0..matrix_size {
            let curr_pixel = self.led_array.get(i).unwrap();
            let color: u32 =
                ((curr_pixel.g as u32) << 16) | ((curr_pixel.r as u32) << 8) | curr_pixel.b as u32;

            for i in (0..24).rev() {
                let pos = 1_u32 << i;
                let bit = pos & color != 0;
                let (high_pulse, low_pulse) = if bit { (t1h, t1l) } else { (t0h, t0l) };
                signal.push(std::iter::once(&high_pulse))?;
                signal.push(std::iter::once(&low_pulse))?;
            }
        }
        self.tx_driver.start_blocking(&signal)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB8 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB8 { r, g, b }
    }
}

trait NanoSeconds {
    fn ns(self) -> Duration;
}

impl NanoSeconds for u64 {
    fn ns(self) -> Duration {
        Duration::from_nanos(self)
    }
}
