use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use x11::xlib;

use std::cell::RefCell;
use std::panic::AssertUnwindSafe;

thread_local! {
    /// Used as part of [`XerrorHandler::handle()`]. When an X11 error occurs during this function,
    /// the error gets copied to this RefCell after which the program is allowed to resume. The
    /// error can then be converted to a regular Rust Result value afterwards.
    static CURRENT_X11_ERROR: RefCell<Option<xlib::XErrorEvent>> = RefCell::new(None);
}

/// A helper struct for safe X11 error handling
pub struct XErrorHandler<'a> {
    display: *mut xlib::Display,
    error: &'a RefCell<Option<xlib::XErrorEvent>>,
}

impl<'a> XErrorHandler<'a> {
    /// Syncs and checks if any previous X11 calls returned an error
    pub fn check(&mut self) -> Result<(), XLibError> {
        // Flush all possible previous errors
        unsafe {
            xlib::XSync(self.display, 0);
        }
        let error = self.error.borrow_mut().take();

        match error {
            None => Ok(()),
            Some(inner) => Err(XLibError::from_event(inner)),
        }
    }

    /// Sets up a temporary X11 error handler for the duration of the given closure, and allows
    /// that closure to check on the latest X11 error at any time
    pub fn handle<T, F: FnOnce(&mut XErrorHandler) -> T>(
        display: *mut xlib::Display, handler: F,
    ) -> T {
        unsafe extern "C" fn error_handler(
            _dpy: *mut xlib::Display, err: *mut xlib::XErrorEvent,
        ) -> i32 {
            // SAFETY: the error pointer should be safe to copy
            let err = *err;

            CURRENT_X11_ERROR.with(|error| {
                let mut error = error.borrow_mut();
                match error.as_mut() {
                    // If multiple errors occur, keep the first one since that's likely going to be the
                    // cause of the other errors
                    Some(_) => 1,
                    None => {
                        *error = Some(err);
                        0
                    }
                }
            })
        }

        // Flush all possible previous errors
        unsafe {
            xlib::XSync(display, 0);
        }

        CURRENT_X11_ERROR.with(|error| {
            // Make sure to clear any errors from the last call to this function
            *error.borrow_mut() = None;

            let old_handler = unsafe { xlib::XSetErrorHandler(Some(error_handler)) };
            let panic_result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                let mut h = XErrorHandler { display, error };
                handler(&mut h)
            }));
            // Whatever happened, restore old error handler
            unsafe { xlib::XSetErrorHandler(old_handler) };

            match panic_result {
                Ok(v) => v,
                Err(e) => std::panic::resume_unwind(e),
            }
        })
    }
}

/// An owned snapshot of an X11 error.
///
/// The human-readable message is resolved eagerly from the display connection when the
/// error is captured, so the struct holds no raw pointers. That keeps it `Send + Sync`,
/// which lets it travel through `anyhow` as a captured error source.
pub struct XLibError {
    error_code: u8,
    error_message: String,
    minor_code: u8,
    request_code: u8,
    type_: i32,
    resource_id: xlib::XID,
    serial: xlib::XID,
}

impl XLibError {
    fn from_event(event: xlib::XErrorEvent) -> Self {
        Self {
            error_code: event.error_code,
            error_message: Self::resolve_message(event.display, event.error_code),
            minor_code: event.minor_code,
            request_code: event.request_code,
            type_: event.type_,
            resource_id: event.resourceid,
            serial: event.serial,
        }
    }

    fn resolve_message(display: *mut xlib::Display, error_code: u8) -> String {
        let mut buf = [0u8; 255];
        unsafe {
            xlib::XGetErrorText(
                display,
                error_code.into(),
                buf.as_mut_ptr().cast(),
                (buf.len() - 1) as i32,
            );
        }

        *buf.last_mut().unwrap() = 0;
        // SAFETY: whatever XGetErrorText did or not, we guaranteed there is a nul byte at the
        // end of the buffer.
        let message = unsafe { CStr::from_ptr(buf.as_ptr().cast()) };
        message.to_string_lossy().into_owned()
    }
}

impl Debug for XLibError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XLibError")
            .field("error_code", &self.error_code)
            .field("error_message", &self.error_message)
            .field("minor_code", &self.minor_code)
            .field("request_code", &self.request_code)
            .field("type", &self.type_)
            .field("resource_id", &self.resource_id)
            .field("serial", &self.serial)
            .finish()
    }
}
