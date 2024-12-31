// SPDX-License-Identifier: Zlib
use crate::OsType;
use curl::Error as CurlError;
use partialzip::PartialZipError;
use std::io::{Error as IoError, IntoInnerError};
use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum ByondGetError {
	#[error("Invalid BYOND version: {version}.{build} ({os})", os = os.as_str())]
	BadVersion {
		version: u16,
		build: u16,
		os: OsType,
	},
	#[error("Attempted to convert parse OS string \"{0}\"")]
	BadOs(String),
	#[error("CURL Error: {0}")]
	Curl(#[from] CurlError),
	#[error("Error downloading partial BYOND zip: {0}")]
	PartialZip(PartialZipError),
	#[error("Error extracting BYOND zip: {0}")]
	Zip(#[from] ZipError),
	#[error("I/O error: {0}")]
	Io(#[from] IoError),
}

impl From<PartialZipError> for ByondGetError {
	fn from(err: PartialZipError) -> Self {
		match err {
			PartialZipError::InvalidUrl => unreachable!(
				"Somehow got a BadVersion error from a PartialZipError, even though we should \
				 handle that case beforehand!"
			),
			PartialZipError::IOError(err) => ByondGetError::Io(err),
			PartialZipError::CURLError(err) => ByondGetError::Curl(err),
			_ => ByondGetError::PartialZip(err),
		}
	}
}

impl<Writer> From<IntoInnerError<Writer>> for ByondGetError {
	fn from(err: IntoInnerError<Writer>) -> Self {
		ByondGetError::Io(err.into_error())
	}
}
