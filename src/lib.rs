// SPDX-License-Identifier: Zlib
mod download;
mod error;
mod os;

pub use self::{
	download::{download_bin, download_full},
	error::ByondGetError,
	os::OsType,
};

pub type Result<T> = std::result::Result<T, ByondGetError>;
