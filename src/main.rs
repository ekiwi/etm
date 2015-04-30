#![feature(path_ext)]
#![feature(collections)]
#![feature(convert)]

mod usb;

fn main() {
	println!("Welcome to emt version 0.1");

	let dev = match usb::Device::from_address("3-1.2") {
		Ok(d)    => d,
		Err(why) => panic!(why)
	};
	println!("{}", dev);
}
