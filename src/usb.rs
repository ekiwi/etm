use std::path::Path;
use std::fs::PathExt;
use std::fmt;

pub struct Device {
	product: String,
	manufacturer : String,
	idProduct: String,
	idVendor: String
}

impl Device {
	pub fn from_path(path: &Path) -> Device {
		if(!path.exists()) { panic!(format!("Path: {:?} does not exist", path)) };
		
		Device {
			product: String::from_str("N/A"),
			manufacturer: String::from_str("N/A"),
			idProduct: String::from_str("0000"),
			idVendor: String::from_str("0000")
		}
	}

	pub fn from_str(s: &str) -> Device {
		let path_str = &format!("/sys/bus/usb/devices/{}/", s);
		let path = Path::new(path_str);
		Device::from_path(&path)
	}
}

impl fmt::Display for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} ({})", self.product, self.manufacturer)
	}
}
