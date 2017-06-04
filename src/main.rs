extern crate libc;
extern crate byteorder;

mod drm;

use drm::device::*;


fn main() {
    let d = Device::open_device("/dev/dri/card0");
    let buf = d.get_capability(DeviveCapability::DumbBuffer);
    println!("dumb buffer: {}", buf);
    d.get_resources();
}
