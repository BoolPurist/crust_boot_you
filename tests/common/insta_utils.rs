pub fn filter_random_tmp_folder_name<'s: 'static>() -> Vec<(&'s str, &'s str)> {
    vec![(r"/tmp/\.[\w]+/", "/[TMP_ROOT]/")]
}
