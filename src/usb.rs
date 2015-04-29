use std::path::Path;
use std::fs::PathExt;
use std::fmt;

pub struct Device<'a> {
	product: &'a str,
	manufacturer : &'a str,
	idProduct: &'a str,
	idVendor: &'a str
}

impl<'a> Device<'a> {
	pub fn from_path(path: &Path) -> Device<'a> {
		if(!path.exists()) { panic!(format!("Path: {:?} does not exist", path)) };
		
		Device {
			product: "N/A",
			manufacturer: "N/A",
			idProduct: "0000",
			idVendor: "0000"
		}
	}

	pub fn from_string(s: &str) -> Device<'a> {
		let path_str = &format!("/sys/bus/usb/devices/{}/", s);
		let path = Path::new(path_str);
		Device::from_path(&path)
	}
}

impl<'a> fmt::Display for Device<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} ({})", self.product, self.manufacturer)
	}
}
