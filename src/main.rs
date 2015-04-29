#![feature(path_ext)]

mod usb;

fn main() {
	println!("Welcome to emt version 0.1");

	let dev = usb::Device::from_string("3-1.2.1");
	println!("{}", dev);
}
