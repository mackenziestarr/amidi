extern crate libc;
extern crate hex;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use libc::{c_int, c_char, c_void};

use std::fmt;
use std::ptr;
use std::ffi::CStr;
use std::str::FromStr;

#[derive(Debug)]
struct HexData(Vec<u8>);

impl FromStr for HexData {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s).map(HexData)
    }
}

/// @todo help message
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

type PmDeviceId = i32;
type PortMidiStream = c_void;

#[derive(Debug)]
#[repr(C)]
struct PmDeviceInfo {
    struct_version : c_int,
    interface      : *const c_char,
    name           : *const c_char,
    input          : c_int,
    output         : c_int,
    opened         : c_int,
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

#[derive(Debug, PartialEq)]
#[repr(C)]
enum PmError {
    NoError,
    NoData,
    GotData,
    HostError,
    InvalidDeviceId,
    InsufficientMemory,
    BufferTooSmall,
    BufferOverflow,
    BadPtr,
    BadData,
    InternalError,
    BufferMaxSize
}



#[link(name = "portmidi")]
extern "C" {
    fn Pm_Initialize() -> PmError;
    fn Pm_Terminate() -> PmError;
    fn Pm_CountDevices() -> c_int;
    fn Pm_GetDeviceInfo(id: PmDeviceId) -> *const PmDeviceInfo;
    fn Pm_Close(stream: *const PortMidiStream) -> PmError;
    fn Pm_OpenOutput( stream : *const PortMidiStream,
                      outputDevice : PmDeviceId,
                      outputDriverInfo : *const c_void,
                      bufferSize : u32,
                      time_proc : *const c_void,
                      time_info : *const c_void,
                      latency : u32 ) -> PmError;

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

fn send_hex(_hex_data: HexData, device_id: PmDeviceId) {
    let stream = ptr::null();
    let err = unsafe {
        Pm_OpenOutput(
            stream,
            device_id,
            ptr::null(),
            1024,
            ptr::null(),
            ptr::null(),
            0
        )
    };
    if err != PmError::NoError {
        panic!("error opening output port: {:?}", err);
    }
    unsafe {
        Pm_Close(stream);
    }
}

fn main() {

    let err = unsafe {
        Pm_Initialize()
    };
    if err != PmError::NoError {
        panic!("failed to initialize: {:?}", err);
    }

    match Opt::from_args() {
        Opt{
            list_devices: true, ..
        } => print_devices(),
        Opt{ hex_data:  Some(hex_data),
             device_id: Some(device_id),
             ..
        } => send_hex(hex_data, device_id),
        _ => println!("ayy"),
    }

    let err = unsafe {
        Pm_Terminate()
    };
    if err != PmError::NoError {
        panic!("failed to terminate: {:?}r");
    }
}
