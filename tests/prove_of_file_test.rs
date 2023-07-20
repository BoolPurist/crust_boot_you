mod common;

use common::named;
use crust_boot_you::prelude::FileManipulator;

use crate::common::TestSetup;

#[named]
#[test]
fn manipulate_to_match_expected() {
    let setup = TestSetup::new(actual_expected!());
    let temp_path = setup.path_to_temp();
    setup
        .os_mani()
        .delete_whole_folder(&temp_path.join("archive"))
        .unwrap();
    setup
        .os_mani()
        .write_file_to(&temp_path.join("table.txt"), &"Table Cases\n")
        .unwrap();

    setup.assert_with_expected();
}
