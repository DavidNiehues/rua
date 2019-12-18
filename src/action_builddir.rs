use crate::pacman;
use crate::rua_environment;
use crate::rua_files::RuaDirs;
use crate::tar_check;
use crate::wrapped;
use std::path::Path;
use std::path::PathBuf;

pub fn action_builddir(dir: &Option<PathBuf>, dirs: &RuaDirs, offline: bool, force: bool) {
	// Set `.` as default dir in case no build directory is provided.
	let dir = match dir {
		Some(path) => &path,
		None => Path::new("."),
	};
	let dir = dir
		.canonicalize()
		.unwrap_or_else(|err| panic!("Cannot canonicalize path {:?}, {}", dir, err));
	let dir_str = dir
		.to_str()
		.unwrap_or_else(|| panic!("{}:{} Cannot parse CLI target directory", file!(), line!()));
	wrapped::build_directory(dir_str, &dirs, offline, force);

	let srcinfo = wrapped::generate_srcinfo(dir_str, dirs).expect("Failed to obtain SRCINFO");
	let ver = srcinfo.version();
	let ext = rua_environment::extension();
	let archive_names = srcinfo.pkgs.iter().map(|package| {
		let arch = if package.arch.contains(&*pacman::PACMAN_ARCH) {
			pacman::PACMAN_ARCH.to_string()
		} else {
			"any".to_string()
		};
		format!("{}-{}-{}{}", package.pkgname, ver, arch, ext)
	});

	for archive_name in archive_names {
		let file = dir.join(archive_name);
		let file_str = file.to_str().expect("Builddir target has unvalid UTF-8");
		tar_check::tar_check(&file, file_str).ok();
	}

	eprintln!("Package built and checked in: {}", dir_str);
	eprintln!("If you want to install the built artifacts, do it manually.");
}
