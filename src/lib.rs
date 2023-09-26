#![cfg(windows)]

pub mod api;
pub mod dll;
pub mod loader;

#[cfg(feature = "gui")]
pub mod gui; //TODO: impl display
