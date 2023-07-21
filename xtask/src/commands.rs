use crust_boot_you::{
    app_traits::{file_manipulator::DevOsFileManipulator, path_provider::get_root_dev},
    prelude::*,
    DevPathProvider,
};

use crate::cli::{TaskCliEntry, TaskSubcommand};

pub fn handle_commands(cli: TaskCliEntry) -> AppResult<String> {
    let paths = DevPathProvider::default();
    let files = DevOsFileManipulator::default().init_system(init_path());
    match cli.subcommands() {
        TaskSubcommand::Init => init(&paths, &files),
        TaskSubcommand::Clear => clear(&files).context("Could clear tmp folder"),
        TaskSubcommand::Reset => reset(&paths, &files),
    }?;

    Ok(String::new())
}

fn init_path() -> PathBuf {
    Path::new(constants::project_root())
        .join("xtask")
        .join("init_data")
}

fn reset(paths: &DevPathProvider, files: &DevOsFileManipulator) -> AppResult {
    match clear(files) {
        Ok(_) | Err(AppIoError::NotFound) => init(paths, files),
        error => error.context("Expected success or folder simply does not exits already"),
    }
}

fn init(paths: &DevPathProvider, files: &DevOsFileManipulator) -> AppResult {
    let to = paths.root();
    files.ensure_dir(&to)?;
    files.init_copy_to(&to)?;
    Ok(())
}

fn clear(files: &impl FileManipulator) -> AppIoResult {
    let to_delete = get_root_dev();
    files.delete_whole_folder(&to_delete)
}
