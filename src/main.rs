#![feature(path_ext)]
#![feature(collections)]

mod usb;

fn main() {
	println!("Welcome to emt version 0.1");

	let dev = match usb::Device::from_str("3-1.2.1") {
		Ok(d)    => d,
		Err(why) => panic!(why)
	};
	println!("{}", dev);
}
