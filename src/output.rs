use std::ptr;
use portmidi::*;

#[derive(Debug)]
pub struct OutputPort {
    id : PmDeviceId,
    stream : *const PortMidiStream,
}
impl OutputPort {
    pub fn new(id: PmDeviceId) -> Result<Self, PmError> {
        let stream = ptr::null();
        let err = unsafe {
            Pm_OpenOutput(
                &stream, id, ptr::null(), 1024, ptr::null(), ptr::null(), 0
            )
        };

        if err != PmError::NoError {
            return Err(err)
        }

        Ok(OutputPort{id, stream})
    }
    pub fn send_sysex_msg(&self, data: &[u8]) -> Result<(), PmError> {
        match unsafe { Pm_WriteSysEx(self.stream, 0, data.as_ptr()) } {
            PmError::NoError => Ok(()),
            err => Err(err)
        }
    }
}

impl Drop for OutputPort {
    fn drop(&mut self) {
        unsafe { Pm_Close(self.stream); }
    }
}
