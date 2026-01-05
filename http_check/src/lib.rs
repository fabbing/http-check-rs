pub mod check;
pub mod config;
pub mod version;

pub use anyhow::{Result, anyhow, bail};
pub type GenericError = anyhow::Error;
