// SPDX-License-Identifier: Zlib
use crate::{ByondGetError, OsType, Result};
use curl::easy::Easy;
use partialzip::{PartialZip, PartialZipError};
use std::{
	fs::File,
	io::{BufWriter, Cursor},
	path::Path,
};
use zip::ZipArchive;

pub fn download_full(version: u16, build: u16, os: OsType, path: impl AsRef<Path>) -> Result<()> {
	let path = path.as_ref();
	let mut zip = Vec::<u8>::with_capacity(16 * 1024 * 1024); // 16 MiB is a comfy margin of error
	let mut handle = Easy::new();
	handle.url(&os.url(version, build))?;
	handle.fail_on_error(true)?;
	{
		let mut transfer = handle.transfer();
		transfer.write_function(|data| {
			zip.extend_from_slice(data);
			Ok(data.len())
		})?;
		transfer.perform().map_err(|err| {
			if err.is_http_returned_error() {
				ByondGetError::BadVersion { version, build, os }
			} else {
				ByondGetError::Curl(err)
			}
		})?;
	}
	let mut zip = ZipArchive::new(Cursor::new(zip))?;
	for idx in 0..zip.len() {
		let mut entry = zip.by_index(idx)?;
		if !entry.is_file() {
			continue;
		}
		let zip_path = match entry.enclosed_name() {
			Some(path) => path,
			None => continue,
		};
		let stripped_path = match zip_path.strip_prefix("byond/") {
			Ok(stripped_path) => stripped_path,
			Err(_) => continue,
		};
		let extract_path = path.join(stripped_path);
		if let Some(parent_dir) = extract_path.parent().filter(|parent| !parent.exists()) {
			std::fs::create_dir_all(parent_dir)?;
		}
		let mut file = File::create(&extract_path).map(BufWriter::new)?;
		std::io::copy(&mut entry, &mut file)?;
		file.into_inner()?.sync_all()?;

		#[cfg(unix)]
		if !file_path.ends_with(".so") {
			use std::{fs::Permissions, os::unix::fs::PermissionsExt};
			let mut perms = std::fs::metadata(path)?.permissions();
			perms.set_mode(perms.mode() | 0o111);
			std::fs::set_permissions(download_path, perms)?;
		}
	}
	Ok(())
}

pub fn download_bin(version: u16, build: u16, os: OsType, path: impl AsRef<Path>) -> Result<()> {
	let path = path.as_ref();
	let url = os.url(version, build);
	let zip = PartialZip::new(&url).map_err(|err| match err {
		PartialZipError::InvalidUrl => ByondGetError::BadVersion { version, build, os },
		_ => err.into(),
	})?;
	let zip_files = zip.list_names();
	for (zip_path, file_path) in zip_files
		.iter()
		.filter(|path| !path.ends_with('/'))
		.filter_map(|zip_path| {
			zip_path
				.strip_prefix("byond/bin/")
				.map(|stripped_path| (zip_path.as_ref(), stripped_path))
		}) {
		let download_path = path.join(file_path);
		if let Some(parent_dir) = download_path.parent().filter(|parent| !parent.exists()) {
			std::fs::create_dir_all(parent_dir)?;
		}
		let mut file = File::create(&download_path).map(BufWriter::new)?;
		zip.download_to_write(zip_path, &mut file)?;
		file.into_inner()?.sync_all()?;

		#[cfg(unix)]
		if !file_path.ends_with(".so") {
			use std::{fs::Permissions, os::unix::fs::PermissionsExt};
			let mut perms = std::fs::metadata(path)?.permissions();
			perms.set_mode(perms.mode() | 0o111);
			std::fs::set_permissions(download_path, perms)?;
		}
	}
	Ok(())
}
