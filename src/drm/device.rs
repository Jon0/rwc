use std::ffi::CString;
use libc::*;
use drm::ffi::xf86_drm::*;
use drm::ffi::xf86_drm_mode::*;


pub struct Device {
    fd: c_int,
}


impl Device {
    pub fn open_device(name: &str) -> Device {
        let devname = CString::new(name).unwrap();
        unsafe {
            let fd = open(devname.as_ptr(), O_RDWR);
            return Device{ fd: fd };
        }
    }


    pub fn get_resources(&self) {
        unsafe {
            let res = drmModeGetResources(self.fd);
            for i in 0..(*res).count_connectors as isize {
                let conn = drmModeGetConnector(self.fd, *(*res).connectors.offset(i));
            }
        }
    }
}



fn list_devices() {
    let devices = [drmDevice::default()];
    let devices_ptr = devices.as_ptr() as *mut drmDevice;
    unsafe {
        drmGetDevices(&devices_ptr, 1);
    }
}
