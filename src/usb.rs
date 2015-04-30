use std::error::Error;
use std::fs::File;
use std::path::{ Path, PathBuf };
use std::fs::PathExt;
use std::fmt;
use std::io::Read;

extern crate glob;

#[derive(Debug, PartialEq)]
pub enum DeviceType {
	Tty,
	Debugger,
	Unknown
}

pub struct Device {
	product: String,
	manufacturer : String,
	idProduct: String,
	idVendor: String,
	address: String,
	device_type: DeviceType,
	tty_path: String
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

	pub fn from_path(path: &Path, device_type: DeviceType) -> Result<Device, String> {
		if(!path.exists()) { return Err(format!("Path: {:?} does not exist", path)) };

		let mut d = Device {
			product: String::new(),
			manufacturer: String::new(),
			idProduct: String::new(),
			idVendor: String::new(),
			address: String::new(),
			device_type: device_type,	// remember what kind of device this is supposed to be
			tty_path: String::new()
		};

		match Device::read_file(path, "product",      &mut d.product)
			{ Err(why) => return Err(why), Ok(_) => () };
		match Device::read_file(path, "manufacturer", &mut d.manufacturer)
			{ Err(why) => return Err(why), Ok(_) => () };
		match Device::read_file(path, "idProduct",    &mut d.idProduct)
			{ Err(why) => return Err(why), Ok(_) => () };
		match Device::read_file(path, "idVendor",     &mut d.idVendor)
			{ Err(why) => return Err(why), Ok(_) => () };
		// address = busnum + '-' + devpath
		match Device::read_file(path, "busnum", &mut d.address)
			{ Err(why) => return Err(why), Ok(_) => () };
		d.address.push('-');
		let mut devpath = String::new();
		match Device::read_file(path, "devpath", &mut devpath)
			{ Err(why) => return Err(why), Ok(_) => () };
		d.address.push_str(&devpath);

		if(d.device_type == DeviceType::Tty) {
			match d.find_tty_path() {
				Some(path) => d.tty_path = path,
				None => return Err(format!("Failed to find tty path for {}", d.product))
			}
		}

		Ok(d)
	}

	pub fn from_address(address: &str, device_type: DeviceType) -> Result<Device, String> {
		let path_str = &format!("/sys/bus/usb/devices/{}/", address);
		let path = Path::new(path_str);
		Device::from_path(&path, device_type)
	}


	/// tries to convert a sysfs path to the corresponting /dev/tty* path.
	fn tty_path_from_sysfs_path(path: &PathBuf) -> Option<String> {
		match path.file_name() {
			Some(name) =>
				match name.to_str() {
					Some(name_str) => Some(format!("/dev/{}", name_str)),
					None => None
				},
			None => None
		}
	}

	/// this tries to determine the tty path by looking for tty
	/// in the directories for the device interfaces
	/// Examples:
	/// HL-340:       /sys/bus/usb/devices/2-9/2-9:1.0/ttyUSB0
	/// CP210x:       /sys/bus/usb/devices/3-1.2/3-1.2:1.0/ttyUSB0
	/// Atmel XPlain: /sys/bus/usb/devices/3-1.2/3-1.2:1.1/tty/ttyACM0
	fn find_tty_path(&self) -> Option<String> {
		let tty_acm_glob_path_str =
			&format!("/sys/bus/usb/devices/{}/*/tty/ttyACM*", self.address);
		let tty_usb_glob_path_str =
			&format!("/sys/bus/usb/devices/{}/*/ttyUSB*", self.address);

		for path in glob::glob(tty_acm_glob_path_str).unwrap().filter_map(Result::ok) {
			return Device::tty_path_from_sysfs_path(&path)
		}
		for path in glob::glob(tty_usb_glob_path_str).unwrap().filter_map(Result::ok) {
			return Device::tty_path_from_sysfs_path(&path)
		}

		None
	}
}

impl fmt::Display for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.device_type {
			DeviceType::Tty =>
				write!(f, "{}:{} {} [{:?}] @ {} -> {}", self.idVendor,
					self.idProduct, self.product, self.device_type, self.address, self.tty_path),
			_ =>
				write!(f, "{}:{} {} [{:?}] @ {}", self.idVendor,
					self.idProduct, self.product, self.device_type, self.address)
		}
	}
}
