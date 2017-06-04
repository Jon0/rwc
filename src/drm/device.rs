use std::ffi::CString;
use libc::*;
use drm::ffi::drm::*;
use drm::ffi::xf86_drm::*;
use drm::ffi::xf86_drm_mode::*;


pub enum DeviveCapability {
    DumbBuffer         = 0x1,
    VBlankHighCRTC     = 0x2,
    DumbPreferredDepth = 0x3,
    DumbPreferShadow   = 0x4,
    Prime              = 0x5,
    TimestampMonotonic = 0x6,
    AsyncPageFlip      = 0x7,
    CursorWidth        = 0x8,
    CursorHeight       = 0x9,
    AddFB2Modifiers    = 0x10,
}


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


    pub fn get_capability(&self, cap: DeviveCapability) -> u64 {
        let mut result: u64 = 0;
        unsafe {
            let err = drmGetCap(self.fd, cap as u64, &mut result);
        }
        return result;
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
