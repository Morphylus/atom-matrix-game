use esp_idf_svc::{
    hal::{
        delay::Delay,
        gpio::AnyIOPin,
        i2c::{I2c, I2cConfig, I2cDriver},
        peripheral::Peripheral,
        units::Hertz,
    },
    sys::TickType_t,
};

const MPU_ADDR: u8 = 0x68;

const MPU_PWR_MGMT_1: u8 = 0x6b;
const MPU_PWR_MGMT_2: u8 = 0x6c;

const MPU_ACCELL_CONFIG: u8 = 0x1c;
const MPU_GYRO_CONFIG: u8 = 0x1b;
const MPU_CONFIG: u8 = 0x1a;

const MPU_ACCELL_DATA: u8 = 0x3b;
pub struct MPU6886<'a> {
    i2c_driver: I2cDriver<'a>,
}

impl<'d> MPU6886<'d> {
    pub fn new(
        i2c: impl Peripheral<P = impl I2c> + 'd,
        sda: AnyIOPin,
        scl: AnyIOPin,
        baudrate: Hertz,
    ) -> Self {
        let i2c_config = I2cConfig::new().baudrate(baudrate);
        let i2c_driver = I2cDriver::new(i2c, sda, scl, &i2c_config).unwrap();
        MPU6886 { i2c_driver }
    }

    pub fn init(&mut self) {
        let delay: Delay = Default::default();

        // Overwrite with zero
        self.mpu_write_data(MPU_PWR_MGMT_1, 0x00);
        delay.delay_ms(10);

        // Reset device
        self.mpu_write_data(MPU_PWR_MGMT_1, 0x01 << 7);
        delay.delay_ms(10);

        // Start clock
        self.mpu_write_data(MPU_PWR_MGMT_1, 0x01);
        delay.delay_ms(10);

        // Configure accelometer with 8g
        self.mpu_write_data(MPU_ACCELL_CONFIG, 0x10);
        delay.delay_ms(1);

        // Configure gyro with 2000dps
        self.mpu_write_data(MPU_GYRO_CONFIG, 0x18);
        delay.delay_ms(1);
    }

    pub fn get_acc_x(&mut self) -> f32 {
        self.get_acc_data().x
    }

    pub fn get_acc_y(&mut self) -> f32 {
        self.get_acc_data().y
    }

    pub fn get_acc_z(&mut self) -> f32 {
        self.get_acc_data().z
    }

    pub fn get_acc_data(&mut self) -> AccData {
        let mut acc_data_buffer = [0_u8; 6];
        self.mpu_read_data(MPU_ACCELL_DATA, &mut acc_data_buffer);

        let acc_x = ((acc_data_buffer[0] as i16) << 8 | acc_data_buffer[1] as i16) as f32 / 4096.0;
        let acc_y = ((acc_data_buffer[2] as i16) << 8 | acc_data_buffer[3] as i16) as f32 / 4096.0;
        let acc_z = ((acc_data_buffer[4] as i16) << 8 | acc_data_buffer[5] as i16) as f32 / 4096.0;

        AccData {
            x: acc_x,
            y: acc_y,
            z: acc_z,
        }
    }

    fn mpu_write_data(&mut self, register_address: u8, data: u8) {
        self.i2c_driver
            .write(MPU_ADDR, &[register_address, data], TickType_t::MAX)
            .unwrap();
    }

    fn mpu_read_data(&mut self, register_address: u8, buffer: &mut [u8]) {
        self.i2c_driver
            .write_read(MPU_ADDR, &[register_address], buffer, TickType_t::MAX)
            .unwrap();
    }
}

pub struct AccData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
