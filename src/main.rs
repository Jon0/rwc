extern crate libc;
extern crate byteorder;

mod drm;

use std::ffi::CString;
use libc::*;
use drm::ffi::xf86_drm::*;


fn open_device(name: &str) {
    let devname = CString::new(name).unwrap();
    unsafe {
        let fd = open(devname.as_ptr(), O_RDWR);
    }
}


fn main() {

    let devices = [drmDevice::default()];
    let devices_ptr = devices.as_ptr() as *mut drmDevice;
    unsafe {
        drmGetDevices(&devices_ptr, 1);
    }
}
