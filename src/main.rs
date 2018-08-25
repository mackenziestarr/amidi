extern crate hex;

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

use std::fmt;
use std::ffi::CStr;
use std::str::FromStr;

mod portmidi;
use portmidi::*;

mod output;
use output::*;

#[derive(Debug)]
struct HexData(pub Vec<u8>);

impl FromStr for HexData {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s).map(HexData)
    }
}

/// cli tool for sending data over midi
#[derive(StructOpt, Debug)]
#[structopt(name = "amidi")]
struct Opt {
    /// list available midi devices
    #[structopt(short = "l", long = "list")]
    list_devices: bool,

    /// sends the bytes specified as hexadecimal numbers to the MIDI port.
    #[structopt(short = "S", long = "send-hex")]
    hex_data: Option<HexData>,

    /// device id to send messages to
    #[structopt(short = "D", long = "device-id")]
    device_id: Option<PmDeviceId>,
}

#[derive(Debug)]
enum IO {
    Input,
    Output,
}
impl fmt::Display for IO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           IO::Input => write!(f, "input"),
           IO::Output => write!(f, "output"),
       }
    }
}

#[derive(Debug)]
struct DeviceInfo {
    id : PmDeviceId,
    interface : String,
    name      : String,
    io : IO,
}

impl DeviceInfo {
    fn new(id: PmDeviceId) -> DeviceInfo {
        let PmDeviceInfo{interface, name, input, output, ..}  = unsafe {
            &*Pm_GetDeviceInfo(id)
        };
        DeviceInfo{
            id : id,
            interface : unsafe {
                CStr::from_ptr(*interface).to_str().unwrap().to_string()
            },
            name : unsafe {
                CStr::from_ptr(*name).to_str().unwrap().to_string()
            },
            io : match (input, output) {
                (1, 0) => IO::Input,
                (0, 1) => IO::Output,
                _ => panic!("unexpected I/O configuration")
            }
        }
    }
}


fn print_devices() {
    let count = unsafe {
        Pm_CountDevices()
    };
    if count > 0 {
        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", "id", "name", "interface", "direction");
        for id in 0..count {
            let device = DeviceInfo::new(id);
            println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", device.id, device.name, device.interface, device.io);
        }
    } else {
        println!("no midi devices");
    }

}
    // unsafe {
    //     Pm_Close(stream);
    // }


fn main() {

    let err = unsafe {
        Pm_Initialize()
    };
    if err != PmError::NoError {
        panic!("failed to initialize: {:?}", err);
    }

    match Opt::from_args() {
        Opt{ list_devices: true, ..} => print_devices(),
        Opt{ hex_data:  Some(hex_data), device_id: Some(device_id), ..} => {
            let output = OutputPort::new(device_id).unwrap();
            output.send_sysex_msg(hex_data.0.as_slice()).unwrap();
        }
        _ => ()
    }

    let err = unsafe {
        Pm_Terminate()
    };
    if err != PmError::NoError {
        panic!("failed to terminate: {:?}", err);
    }
}
