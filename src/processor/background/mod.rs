use super::*;

#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod _impl;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod _impl;

pub use _impl::*;
