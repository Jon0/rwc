use libc::*;


#[repr(C)]
pub struct DrmModeRes {
    pub count_fbs: c_int,
    pub fbs: *mut uint32_t,

    pub count_crtcs: c_int,
    pub crtcs: *mut uint32_t,

    pub count_connectors: c_int,
    pub connectors: *mut uint32_t,

    pub count_encoders: c_int,
    pub encoders: *mut uint32_t,

    pub min_width: uint32_t,
    pub max_width: uint32_t,
    pub min_height: uint32_t,
    pub max_height: uint32_t
}
impl ::std::default::Default for DrmModeRes {
    fn default() -> DrmModeRes { unsafe { ::std::mem::zeroed() } }
}


pub type DrmModeResPtr = *mut DrmModeRes;


#[link(name = "drm")]
extern {
    pub fn drmModeGetResources(fd: c_int) -> DrmModeResPtr;
}
