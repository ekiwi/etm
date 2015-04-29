use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::fs::PathExt;
use std::fmt;
use std::io::Read;

pub struct Device {
	product: String,
	manufacturer : String,
	idProduct: String,
	idVendor: String
}

impl Device {
	/// Helper function that tries to read the file and return its content.
	fn read_file(path: &Path, name: &str, out: &mut String) -> Result<(), String> {
		let mut buf = path.to_path_buf();
		buf.push(name);
		let mut file = match File::open(&buf) {
			Err(why) => return Err(format!("Could not open {}: {}", buf.display(), Error::description(&why))),
			Ok(file) => file
		};
		match file.read_to_string(out) {
			Err(why) => return Err(format!("Could not read {}: {}", buf.display(), Error::description(&why))),
			Ok(_)    => ()
		};
		out.pop();
		Ok(())
	}

	pub fn from_path(path: &Path) -> Result<Device, String> {
		if(!path.exists()) { return Err(format!("Path: {:?} does not exist", path)) };

		let mut d = Device {
			product: String::new(),
			manufacturer: String::new(),
			idProduct: String::new(),
			idVendor: String::new()
		};

		match Device::read_file(path, "product",      &mut d.product)
			{ Err(why) => return Err(why), Ok(_) => () };
		match Device::read_file(path, "manufacturer", &mut d.manufacturer)
			{ Err(why) => return Err(why), Ok(_) => () };
		match Device::read_file(path, "idProduct",    &mut d.idProduct)
			{ Err(why) => return Err(why), Ok(_) => () };
		match Device::read_file(path, "idVendor",     &mut d.idVendor)
			{ Err(why) => return Err(why), Ok(_) => () };

		Ok(d)
	}

	pub fn from_str(s: &str) -> Result<Device, String> {
		let path_str = &format!("/sys/bus/usb/devices/{}/", s);
		let path = Path::new(path_str);
		Device::from_path(&path)
	}
}

impl fmt::Display for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} ({}) [{}:{}]", self.product, self.manufacturer, self.idProduct, self.idVendor)
	}
}
