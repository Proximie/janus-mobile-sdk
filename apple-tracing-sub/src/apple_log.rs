use crate::level::Level;
use crate::sys::ats_get_default_log;
use crate::sys::ats_os_log_with_type;
use crate::sys::os_log_create;
use crate::sys::os_log_t;
use crate::sys::os_release;
use crate::utils::to_cstr;
use std::ffi::c_void;

pub struct AppleLog {
    inner: os_log_t,
}

unsafe impl Send for AppleLog {}
unsafe impl Sync for AppleLog {}

impl AppleLog {
    pub fn new(subsystem: &str, category: &str) -> Self {
        let subsystem = to_cstr(subsystem);
        let category = to_cstr(category);
        let inner = unsafe { os_log_create(subsystem.as_ptr(), category.as_ptr()) };
        assert!(!inner.is_null());
        Self { inner }
    }

    pub fn log(&self, message: &str, level: tracing_core::Level) {
        let message = to_cstr(message);
        let level = Level::from(level);
        unsafe {
            ats_os_log_with_type(self.inner, level.into(), message.as_ptr());
        }
    }
}

impl Drop for AppleLog {
    fn drop(&mut self) {
        unsafe {
            if self.inner != ats_get_default_log() {
                os_release(self.inner as *mut c_void);
            }
        }
    }
}
