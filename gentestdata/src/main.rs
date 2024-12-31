use byond_get::OsType;
use glob::glob;

fn main() {
	let versions = std::env::args()
		.skip(1)
		.map(|version_string| {
			let (version, build) = version_string
				.split_once('.')
				.expect("failed to parse version string");
			(
				version.parse().expect("failed to parse byond version"),
				build.parse().expect("failed to parse byond build"),
			)
		})
		.collect::<Vec<(u16, u16)>>();
	for (version, build) in versions {
		get_files(version, build, OsType::Windows);
		get_files(version, build, OsType::Linux);
	}
}

fn get_files(version: u16, build: u16, os: OsType) {
	let tempdir = tempfile::tempdir().expect("failed to create tempdir");
	let temp_path = tempdir.path();
	if let Err(err) = byond_get::download_full(version, build, os, temp_path) {
		panic!(
			"Failed to download BYOND {version}.{build} for {os} from {url}\n{err:?}",
			url = os.url(version, build),
		);
	}
	let mut extracted_files = glob(&format!("{}/**/*", temp_path.display()))
		.expect("failed to get contents of tempdir")
		.filter_map(|entry| entry.ok())
		.filter(|path| path.is_file())
		.map(|path| {
			path.strip_prefix(temp_path)
				.expect("failed to strip prefix")
				.to_string_lossy()
				.trim()
				.replace('\\', "/")
		})
		.collect::<Vec<String>>();
	extracted_files.sort();
	let list_path = format!("tests/{os}/{version}.{build}.txt");
	if let Err(err) = std::fs::write(&list_path, extracted_files.join("\n")) {
		panic!("Failed to write list of extracted files to {list_path}\n{err:?}");
	}
}
