use std::error::Error;
use std::fs::File;
use std::path::{ Path, PathBuf };
use std::fs::PathExt;
use std::fmt;
use std::io::Read;

extern crate glob;

#[derive(Debug)]
pub enum DeviceType {
	Tty,
	Debugger
}

#[derive(Debug)]
pub enum DebuggerType {
	StLinkv2,
	AtmelCmsisDap,
	Unknown
}

pub struct Device {
	product: String,
	manufacturer : String,
	id_product: String,
	id_vendor: String,
	address: String,
	device_type: DeviceType,
	tty_path: String,
	debugger_type: DebuggerType
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
		if !path.exists() { return Err(format!("Path: {:?} does not exist", path)) };

		let mut d = Device {
			product: String::new(),
			manufacturer: String::new(),
			id_product: String::new(),
			id_vendor: String::new(),
			address: String::new(),
			device_type: device_type,	// remember what kind of device this is supposed to be
			tty_path: String::new(),
			debugger_type: DebuggerType::Unknown
		};

		try!(Device::read_file(path, "product",      &mut d.product));
		try!(Device::read_file(path, "manufacturer", &mut d.manufacturer));
		try!(Device::read_file(path, "idProduct",    &mut d.id_product));
		try!(Device::read_file(path, "idVendor",     &mut d.id_vendor));
		// address = busnum + '-' + devpath
		try!(Device::read_file(path, "busnum", &mut d.address));
		d.address.push('-');
		let mut devpath = String::new();
		try!(Device::read_file(path, "devpath", &mut devpath));
		d.address.push_str(&devpath);

		match d.device_type {
			DeviceType::Tty =>
				match d.find_tty_path() {
					Some(path) => d.tty_path = path,
					None => return Err(format!("Failed to find tty path for {}", d))
				},
			DeviceType::Debugger =>
				match d.determin_debugger_type() {
					Some(debugger_type) => d.debugger_type = debugger_type,
					None => return Err(format!("Unknown debugger {}", d))
				}
		}

		Ok(d)
	}

	pub fn from_address(address: &str, device_type: DeviceType) -> Result<Device, String> {
		let path_str = &format!("/sys/bus/usb/devices/{}/", address);
		let path = Path::new(path_str);
		Device::from_path(&path, device_type)
	}

	pub fn get_tty_path(&self) -> &str {
		self.tty_path.as_str()
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

	fn determin_debugger_type(&self) -> Option<DebuggerType> {
		match format!("{}:{}", self.id_vendor, self.id_product).as_str() {
			"0483:3748" => Some(DebuggerType::StLinkv2),
			"to:do" => Some(DebuggerType::AtmelCmsisDap),
			_           => None
		}
	}
}

impl fmt::Display for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.device_type {
			DeviceType::Tty =>
				write!(f, "{}:{} {} [{:?}] @ {} -> {}", self.id_vendor,
					self.id_product, self.product, self.device_type,
					self.address, self.tty_path),
			DeviceType::Debugger =>
				write!(f, "{}:{} {} [{:?}: {:?}] @ {}", self.id_vendor,
					self.id_product, self.product, self.device_type,
					self.debugger_type, self.address)
		}
	}
}
