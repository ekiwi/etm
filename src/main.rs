#![feature(path_ext)]
#![feature(collections)]
#![feature(convert)]

mod usb;

fn main() {
	println!("Welcome to emt version 0.1");

	let dev = match usb::Device::from_address("3-1.2", usb::DeviceType::Tty) {
		Ok(d)    => d,
		Err(why) => panic!(why)
	};
	println!("{}", dev);

	let dev = match usb::Device::from_address("1-2", usb::DeviceType::Debugger) {
		Ok(d)    => d,
		Err(why) => panic!(why)
	};
	println!("{}", dev);
}
