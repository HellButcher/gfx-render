extern crate crossbeam_channel;
#[macro_use]
extern crate failure;
extern crate gfx_hal as hal;
extern crate gfx_memory as mem;
#[macro_use]
extern crate log;
#[cfg(feature = "regex")]
extern crate regex;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(all(feature = "serde", feature = "regex"))]
extern crate serde_regex;
extern crate winit;

#[cfg(feature = "profile")]
extern crate flame;

#[cfg(feature = "profile")]
macro_rules! profile {
    ($name:tt) => {
        let guard = ::flame::start_guard(concat!("'", $name, "' at : ", line!()));
    };
}

#[cfg(not(feature = "profile"))]
macro_rules! profile {
    ($name:tt) => {};
}

#[cfg(feature = "gfx-backend-vulkan")]
pub extern crate gfx_backend_vulkan as vulkan;

#[cfg(feature = "gfx-backend-dx12")]
pub extern crate gfx_backend_dx12 as dx12;

#[cfg(feature = "gfx-backend-metal")]
pub extern crate gfx_backend_metal as metal;

#[cfg(feature = "gfx-backend-empty")]
pub extern crate gfx_backend_empty as backend_empty;

mod backend;
mod escape;
mod factory;
mod init;
mod reclamation;
mod renderer;
mod upload;

pub use backend::BackendEx;
pub use factory::{Buffer, Factory, Image, Item};
pub use init::{init, init_with_instance, Config, MemoryConfig, AdapterPicker, QueuesPicker, FirstAdapter, adapter_picker, adapter_by_name, adapter_by_surface, queue_picker};
#[cfg(feature = "regex")]
pub use init::adapter_by_name_regex;
pub use renderer::{Render, Renderer, TargetId};
