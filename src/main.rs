extern crate hidapi;

use std::thread;

fn main() {
    let api = hidapi::HidApi::new().unwrap();

    // Connect to device using its VID and PID
    let (vid, pid) = (0x24F7, 0x2204);
    let device = match api.open(vid, pid) {
        Err(_) => panic!("couldn't detect a seneye device connected."),
        Ok(result) => result,
    };

    println!(
        "found device -> {}",
        device.get_serial_number_string().unwrap()
    );

    // set the device to non-blocking
    device
        .set_blocking_mode(false)
        .expect("could not set device to non-blocking mode");

    //send hello to the device (not sure why... lol)
    send_hello(&device);

    let mut ctr = 0;

    let mut read_buf: [u8; 64] = [0x00; 64];

    // start a timer loop to get periodic updates from the device
    loop {
        let res = device.read(&mut read_buf).unwrap();
        if res == 64 && read_buf[0] == 0x00 {
            interogate(&device, &mut ctr);
        } else {
            println!("its busy");
        }

        thread::sleep(std::time::Duration::from_secs(5));
    }

    send_bye(&device);
}

fn send_hello(device: &hidapi::HidDevice) {
    send_message(device, "HELLOSUD").unwrap();
}

fn send_bye(device: &hidapi::HidDevice) {
    send_message(device, "BYESUD").unwrap();
}

fn send_message(device: &hidapi::HidDevice, message: &str) -> hidapi::HidResult<usize> {
    let mut buf: [u8; 65] = [0x00; 65];
    for i in 0..message.len() {
        let chr = message.chars().nth(i).unwrap();
        println!("{}", chr);

        buf[i + 1] = chr as u8;
    }
    device.write(&buf)
}

fn interogate(device: &hidapi::HidDevice, ctr: &mut i32) {
    println!("interogate called with {}", ctr);
    let off_or_on = (*ctr % 10) - 5 >= 0;
    let sequence: usize = (*ctr % 5) as usize;
    let mut buf: [u8; 65] = [0x00; 65];
    buf[1] = 'L' as u8;
    buf[2] = 'E' as u8;
    buf[3] = 'D' as u8;
    for i in 0..5 {
        buf[i + 4] = if off_or_on || i < sequence {
            '0' as u8
        } else {
            '1' as u8
        };
    }
    for i in 0..8 {
        println!("{}", buf[i + 1]);
    }
    let res = device.write(&buf).unwrap();
    println!("Wrote: {:?} byte(s)", res);

    *ctr = *ctr + 1;
}
