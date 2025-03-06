pub mod args;

use self::args::Args;
use color_eyre::eyre::{Result, WrapErr};

fn main() -> Result<()> {
	color_eyre::install()?;
	let Args {
		version,
		os,
		output,
		bin,
	} = Args::parse();
	if bin {
		println!(
			"Downloading BYOND {version} (binaries only) for {os} into {output}",
			output = output.display()
		);
		byond_get::download_bin(version.version, version.build, os.into(), &output)
	} else {
		println!(
			"Downloading BYOND {version} for {os} into {output}",
			output = output.display()
		);
		byond_get::download_full(version.version, version.build, os.into(), &output)
	}
	.wrap_err_with(|| format!("Failed to download BYOND {version} for {os}"))?;
	println!("Success!");
	Ok(())
}
