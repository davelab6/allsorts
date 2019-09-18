#![warn(rust_2018_idioms)]

/// Reading and writing of binary data
pub mod binary;
pub mod cff;
/// Checksum calculation routines.
pub mod checksum;
pub mod context;
pub mod error;
pub mod font_data_impl;
pub mod font_tables;
pub mod fontfile;
pub mod gdef;
pub mod get_name;
pub mod glyph_width;
pub mod gpos;
pub mod gsub;
pub mod indic;
pub mod layout;
/// Utilities for handling the Mac OS Roman character set.
pub mod macroman;
pub mod opentype;
pub mod post;
pub mod size;
/// Font subsetting.
pub mod subset;
pub mod tables;
pub mod tag;
/// Shared test code
#[cfg(test)]
pub mod tests;
pub mod woff;
pub mod woff2;

pub use crate::binary::read;