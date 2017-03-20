extern crate libc;
extern crate byteorder;

mod drm;

use drm::ffi::xf86_drm::*;


fn main() {
    let devices = [drmDevice::default()];
    let devices_ptr = devices.as_ptr() as *mut drmDevice;
    unsafe {
        drmGetDevices(&devices_ptr, 1);
    }
}
