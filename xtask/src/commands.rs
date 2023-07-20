use crust_boot_you::{
    app_traits::{file_manipulator::DevOsFileManipulator, path_provider::get_root_dev},
    prelude::*,
    DevPathProvider, LoadedNode,
};

use crate::{
    cli::{TaskCliEntry, TaskSubcommand},
    init_target::InitTarget,
};

pub fn handle_commands(cli: TaskCliEntry) -> AppResult<String> {
    let paths = DevPathProvider;
    let files = DevOsFileManipulator::default();
    match cli.subcommands() {
        TaskSubcommand::Init => init(&paths, &files),
        TaskSubcommand::Clear => clear(&files).context("Could clear tmp folder"),
        TaskSubcommand::Reset => reset(&paths, &files),
    }?;

    Ok(String::new())
}

fn reset(paths: &impl PathProvider, files: &impl FileManipulator) -> AppResult {
    match clear(files) {
        Ok(_) | Err(AppIoError::NotFound) => init(paths, files),
        error => error.context("Expected success or folder simply does not exits already"),
    }
}

fn init(paths: &impl PathProvider, files: &impl FileManipulator) -> AppResult {
    let template = paths.general_template_entry()?;
    let cwd = paths.cwd()?;
    files.ensure_dir(&template)?;
    files.ensure_dir(&cwd)?;
    files.write_file_to(&cwd.join("config.sh"), "echo 1\necho 2")?;
    init_folders_in_cwd(paths, files)?;
    Ok(())
}

fn init_folders_in_cwd(paths: &impl PathProvider, files: &impl FileManipulator) -> AppResult {
    let cwd = paths.cwd()?;
    let raw_init_data = include_str!("../init.ron");
    let folder_names: Vec<InitTarget> = ron::from_str(&raw_init_data).unwrap();

    folder_names
        .into_iter()
        .map(|next_to_init| {
            let root_path = cwd.join(&next_to_init.path());
            files.ensure_dir(&root_path).unwrap();
            let data: Vec<LoadedNode> = next_to_init
                .nodes()
                .into_iter()
                .map(|to_prepend| to_prepend.clone().prepend_root(&root_path))
                .collect();
            data
        })
        .flatten()
        .for_each(|element| files.write_node(element).unwrap());
    Ok(())
}

fn clear(files: &impl FileManipulator) -> AppIoResult {
    let to_delete = get_root_dev();
    files.delete_whole_folder(&to_delete)
}
