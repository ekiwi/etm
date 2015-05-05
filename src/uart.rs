
extern crate serial;
//extern crate time;

use std::env;
use std::io;

//use time::Duration;

use std::io::prelude::*;
use self::serial::prelude::*;


pub enum UartState {
	Closed,
//	Opening,
	Open,
//	Closing
}

pub struct Uart {
	tty_path: String,
	state: UartState,
	baudrate: serial::BaudRate
}


impl Uart {
	pub fn from(tty_path: String, baudrate: String) -> Result<Uart, String> {
		Ok(Uart {
			tty_path: tty_path,
			state: UartState::Closed,
			baudrate: try!(Uart::baudrate_from_string(baudrate))
		})
	}

//	pub fn open(&self) -> Result<(), String> {
//	}

//	pub fn close(&self) -> Result<(), String> {
//	}

	fn baudrate_from_string(string: String) -> Result<serial::BaudRate, String> {
		match string.as_str() {
			"9600"   => Ok(serial::BaudRate::Baud9600),
			"115200" => Ok(serial::BaudRate::Baud115200),
			"230400" => Ok(serial::BaudRate::Baud230400),
			_        => Err(format!("Baudrate `{}` was not understood or \
			                         could not be read. A Valid example \
			                         would be `115200`", string))
		}
	}
}



//fn main() {
//    for arg in env::args_os().skip(1) {
//        println!("opening port: {:?}", arg);
//        let mut port = serial::open(&arg).unwrap();

//        interact(&mut port).unwrap();
//    }
//}

//fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
//    try!(port.configure(|settings| {
//        settings.set_baud_rate(serial::Baud9600);
//        settings.set_char_size(serial::Bits8);
//        settings.set_parity(serial::ParityNone);
//        settings.set_stop_bits(serial::Stop1);
//        settings.set_flow_control(serial::FlowNone);
//    }));

//    port.set_timeout(Duration::milliseconds(1000));

//    let mut buf: Vec<u8> = (0..255).collect();

//    println!("writing bytes");
//    try!(port.write(&buf[..]));

//    println!("reading bytes");
//    try!(port.read(&mut buf[..]));

//    Ok(())
//}




#[cfg(test)]
mod test {
	use std;
	use super::{Uart};
	use uart::serial;
	use uart::serial::prelude::*;

	#[test]
	fn uart_baudrate_from_string() {
		assert_eq!(Uart::baudrate_from_string(String::from("9600")), Ok(serial::BaudRate::Baud9600));
	}
}
