mod common;
use common::insta_utils;
use common::prelude::*;
use crust_boot_you::{
    app_traits::{file_manipulator::DevOsFileManipulator, path_resolver::DevPathResolver},
    cli::{AbsoluteExistingPath, SaveTemplateCli, TemplateCliArg},
    handle_commands, DevPathProvider,
};

#[test]
#[named]
fn save_file_new_template() {
    let setup = TestSetup::new(actual_expected!());
    let tmp_path = setup.path_to_temp();
    let path_resol = DevPathResolver::new(tmp_path.to_path_buf());
    let path_provider = DevPathProvider::new(tmp_path.to_path_buf());
    let file_sys = DevOsFileManipulator::new(tmp_path);

    let given_name = ValidTemplateName::new("a".to_string()).unwrap();
    let given_path = AbsoluteExistingPath::new(PathBuf::from("new.txt"), &path_resol).unwrap();
    let arguments: SaveTemplateCli = TemplateCliArg::new(given_name, given_path).into();
    let output =
        handle_commands::handle_save_template(path_provider, file_sys, &arguments).unwrap();
    insta::with_settings!({ filters => insta_utils::filter_random_tmp_folder_name() }, { insta::assert_display_snapshot!(output) });

    setup.assert_with_expected();
}
