fn init(paths: &impl PathProvider, files: &impl FileManipulator) -> AppResult {
    let template = paths.general_template_entry()?;
    let cwd = paths.cwd()?;
    files.ensure_dir(&template)?;
    files.ensure_dir(&cwd)?;
    files.write_file_to(&cwd.join("config.sh"), "echo 1\necho 2")?;
    init_folders_in_cwd(paths, files)?;
    Ok(())
}
