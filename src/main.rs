extern crate hidapi;

use std::thread;

mod seneye_reader;

fn main() {
    let api = hidapi::HidApi::new().unwrap();

    let seneye_controller = seneye_reader::SeneyeController::new(&api).unwrap();

    // start a timer loop to get periodic updates from the device
    loop {
        // let res = device.read(&mut read_buf).unwrap();
        // if res == 64 && read_buf[0] == 0x00 {
        //     //interogate(&device, &mut ctr);
        // } else {
        //     println!("its busy");
        // }

        thread::sleep(std::time::Duration::from_secs(5));
    }
}

// fn interogate(device: &hidapi::HidDevice, ctr: &mut i32) {
//     println!("interogate called with {}", ctr);
//     let off_or_on = (*ctr % 10) - 5 >= 0;
//     let sequence: usize = (*ctr % 5) as usize;
//     let mut buf: [u8; 65] = [0x00; 65];
//     buf[1] = 'L' as u8;
//     buf[2] = 'E' as u8;
//     buf[3] = 'D' as u8;
//     for i in 0..5 {
//         buf[i + 4] = if off_or_on || i < sequence {
//             '0' as u8
//         } else {
//             '1' as u8
//         };
//     }
//     for i in 0..8 {
//         println!("{}", buf[i + 1]);
//     }
//     let res = device.write(&buf).unwrap();
//     println!("Wrote: {:?} byte(s)", res);

//     *ctr = *ctr + 1;
// }
