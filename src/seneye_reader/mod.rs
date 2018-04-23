extern crate hidapi;

pub struct SeneyeController<'a> {
    device: hidapi::HidDevice<'a>,
    //is_busy: bool,
}

pub struct LightMeterResult {
    pub is_kelvin: bool,
    pub kelvin: u32,
    pub cie_x: u32, // div by 10000
    pub cie_y: u32, // div by 10000
    pub par: u32,
    pub lux: u32,
    pub pur: u32,
}

pub struct ReadResult {
    pub timestamp: i64,
    pub lm_result: LightMeterResult,
    pub in_water: bool,
    pub slide_not_fitted: bool,
    pub slide_expired: bool,
    pub is_error: bool,
    pub ph: u8,    // div by 100
    pub nh3: u8,   // div by 1000
    pub temp: i32, // div by 1000
}

impl<'a> SeneyeController<'a> {
    pub fn new(api: &'a hidapi::HidApi) -> Result<SeneyeController<'a>, &'a str> {
        // Connect to device using its VID and PID
        let (vid, pid) = (0x24F7, 0x2204);
        let device = match api.open(vid, pid) {
            Err(_) => return Err("couldn't detect a seneye device connected."),
            Ok(result) => result,
        };

        // set the device to non-blocking
        device
            .set_blocking_mode(false)
            .expect("could not set device to non-blocking mode");

        Ok(SeneyeController { device: device })
    }

    fn send_hello(&self) -> Result<LightMeterResult, &str> {
        self.send_message("HELLOSUD").unwrap();
        Err("lol")
    }

    // fn send_bye(device: &hidapi::HidDevice) {
    //     send_message(device, "BYESUD").unwrap();
    // }

    fn send_message(&self, message: &str) -> hidapi::HidResult<usize> {
        let mut buf: [u8; 65] = [0x00; 65];
        for i in 0..message.len() {
            let chr = message.chars().nth(i).unwrap();
            println!("{}", chr);

            buf[i + 1] = chr as u8;
        }
        self.device.write(&buf)
    }
}
