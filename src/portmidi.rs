extern crate libc;
use self::libc::{c_int, c_char, c_void};

pub type PmDeviceId = i32;
pub type PmTimestamp = i32;
pub type PortMidiStream = c_void;

#[link(name = "portmidi")]
extern "C" {
    pub fn Pm_Initialize() -> PmError;
    pub fn Pm_Terminate() -> PmError;
    pub fn Pm_CountDevices() -> c_int;
    pub fn Pm_GetDeviceInfo(id: PmDeviceId) -> *const PmDeviceInfo;
    pub fn Pm_Close(stream: *const PortMidiStream) -> PmError;
    pub fn Pm_OpenOutput( stream : *const *const PortMidiStream,
                      outputDevice : PmDeviceId,
                      outputDriverInfo : *const c_void,
                      bufferSize : u32,
                      time_proc : *const c_void,
                      time_info : *const c_void,
                      latency : u32 ) -> PmError;
    pub fn Pm_WriteSysEx( stream : *const PortMidiStream,
                      when : PmTimestamp,
                      msg : *const u8 ) -> PmError;
}

#[derive(Debug, PartialEq)]
#[repr(C)]
#[allow(dead_code)]
pub enum PmError {
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

#[derive(Debug)]
#[repr(C)]
pub struct PmDeviceInfo {
    pub struct_version : c_int,
    pub interface      : *const c_char,
    pub name           : *const c_char,
    pub input          : c_int,
    pub output         : c_int,
    pub opened         : c_int,
}
