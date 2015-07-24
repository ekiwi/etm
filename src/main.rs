#![feature(path_ext)]
#![feature(convert)]

mod usb;
mod uart;

fn main() {
	println!("Welcome to emt version 0.1");

	let dev = match usb::Device::from_address("1-1.1", usb::DeviceType::Debugger) {
		Ok(d)    => d,
		Err(why) => panic!(why)
	};
	println!("{}", dev);

	let dev = match usb::Device::from_address("1-1.4.2", usb::DeviceType::Tty) {
		Ok(d)    => d,
		Err(why) => panic!(why)
	};
	println!("{}", dev);

	let tty = uart::Uart::from(dev.get_tty_path(), "115200");
	tty.start_server("127.0.0.1:5555");
}
