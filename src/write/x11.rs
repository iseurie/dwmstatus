use ::x11::xlib;
use ::std::ptr;
use ::std::ffi::{CString, NulError};

pub struct X11 {
    display: *mut xlib::Display
}

impl X11 {
    pub fn new() -> Option<Self> {
        let dpy: *mut xlib::Display;
        unsafe {
            dpy = xlib::XOpenDisplay(ptr::null());
        }
        if dpy.is_null() { return None }
        Some(Self { display: dpy })
    }

    pub fn with_dpy(name: &str) -> Option<Result<Self, NulError>> {
        let c_name = CString::new(name);
        if c_name.is_err() { return Some(Err(c_name.err().unwrap())) }
        let dpy: *mut xlib::Display;
        unsafe {
            dpy = xlib::XOpenDisplay(c_name.unwrap().as_ptr());
        }
        if dpy.is_null() { return None }
        Some(Ok(Self { display: dpy }))
    }
}

impl super::WriteStatus for X11 {
    fn write_status(&mut self, status: &str) {
        use self::xlib::*;
        unsafe {
            XStoreName(
                    self.display,
                    XDefaultRootWindow(self.display),
                    CString::new(status).unwrap().as_ptr()
            );
            XSync(self.display, False);
        }
    }
}
