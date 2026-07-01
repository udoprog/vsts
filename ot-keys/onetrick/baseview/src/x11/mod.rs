mod xcb_connection;
use xcb_connection::XcbConnection;

/// Backend result type for the X11 windowing code.
///
/// Uses [`anyhow`] so the various error types coming out of `xcb`, `nix`, and the
/// standard library compose directly with `?`, and so failures can be annotated with
/// context as they propagate up to the window thread where they are logged.
pub(crate) use anyhow::Result;

mod window;
pub use window::*;

mod cursor;
mod keyboard;
