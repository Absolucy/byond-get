// SPDX-License-Identifier: Zlib
use blake3::Hash;
use byond_get::OsType;
use datatest_stable::Utf8Path;
use glob::glob;
use std::collections::HashMap;

fn get_expected_paths(list: &str, full: bool) -> HashMap<String, Hash> {
	list.trim()
		.lines()
		.map(|path| path.replace('\\', "/"))
		.filter_map(|path| {
			if full {
				Some(path)
			} else {
				path.strip_prefix("bin/").map(str::to_owned)
			}
		})
		.map(|path| {
			let (path, hash_hex) = path.split_once('\t').unwrap();
			(path.to_owned(), Hash::from_hex(hash_hex).unwrap())
		})
		.collect::<HashMap<String, Hash>>()
}

fn download_and_list_files(
	version: u16,
	build: u16,
	os: OsType,
	full: bool,
) -> HashMap<String, Hash> {
	let temp_dir = tempfile::tempdir().expect("failed to create temporary directory");
	let temp_path = temp_dir.path();
	if full {
		byond_get::download_full(version, build, os, temp_path)
			.expect("failed to download and extract BYOND");
	} else {
		byond_get::download_bin(version, build, os, temp_path)
			.expect("failed to download and extract bin-only BYOND");
	}
	glob(&format!("{}/**/*", temp_path.display()))
		.expect("failed to get contents of tempdir")
		.filter_map(|entry| entry.ok())
		.filter(|path| path.is_file())
		.map(|path| {
			let stripped_path = path
				.strip_prefix(temp_path)
				.expect("failed to strip prefix")
				.to_string_lossy()
				.trim()
				.replace('\\', "/");
			let hash = blake3::hash(&std::fs::read(path).expect("failed to read file"));
			(stripped_path, hash)
		})
		.collect::<HashMap<String, Hash>>()
}

fn verify_download(
	path: &Utf8Path,
	contents: String,
	os: OsType,
	full: bool,
) -> datatest_stable::Result<()> {
	let (version, build) = {
		let (version, build) = path
			.file_stem()
			.unwrap()
			.split_once('.')
			.expect("failed to parse version string");
		(
			version
				.parse::<u16>()
				.expect("failed to parse byond version"),
			build.parse::<u16>().expect("failed to parse byond build"),
		)
	};
	let expected_files = get_expected_paths(&contents, full);
	assert_eq!(
		expected_files,
		download_and_list_files(version, build, os, full),
		"extracted files for {version}.{build} on {os} do not match expected files"
	);
	Ok(())
}

fn verify_download_windows_full(path: &Utf8Path, contents: String) -> datatest_stable::Result<()> {
	verify_download(path, contents, OsType::Windows, true)
}

fn verify_download_windows_bin(path: &Utf8Path, contents: String) -> datatest_stable::Result<()> {
	verify_download(path, contents, OsType::Windows, false)
}

fn verify_download_linux_full(path: &Utf8Path, contents: String) -> datatest_stable::Result<()> {
	verify_download(path, contents, OsType::Linux, true)
}

fn verify_download_linux_bin(path: &Utf8Path, contents: String) -> datatest_stable::Result<()> {
	verify_download(path, contents, OsType::Linux, false)
}

datatest_stable::harness! {
	{ test = verify_download_windows_full, root = "tests/windows" },
	{ test = verify_download_windows_bin, root = "tests/windows" },
	{ test = verify_download_linux_full, root = "tests/linux" },
	{ test = verify_download_linux_bin, root = "tests/linux" },
}
