use crate::{
    app_traits::{
        path_provider::{self},
        path_resolver::DevPathResolver,
    },
    file_management::NodeEntryMeta,
    prelude::*,
};

use super::OsFileManipulator;
#[derive(Debug)]
pub struct DevOsFileManipulator {
    init_system: Option<PathBuf>,
    os_impl: OsFileManipulator,
    cwd: PathBuf,
    resolver: DevPathResolver,
}

impl Default for DevOsFileManipulator {
    fn default() -> Self {
        Self {
            init_system: None,
            cwd: path_provider::get_root_default_cwd(),
            os_impl: Default::default(),
            resolver: Default::default(),
        }
    }
}

impl DevOsFileManipulator {
    pub fn new(allowed_root: &Path) -> Self {
        let (cwd, for_resolver) = (allowed_root.to_path_buf(), allowed_root.to_path_buf());
        Self {
            init_system: None,
            cwd,
            os_impl: OsFileManipulator::default(),
            resolver: DevPathResolver::new(for_resolver),
        }
    }

    pub fn init_system(mut self, init_system: PathBuf) -> Self {
        self.init_system = Some(init_system);
        self
    }

    pub fn init_copy_to(&self, to: &Path) -> AppResult {
        match &self.init_system {
            None => panic!("There is not init system to load from !"),
            Some(from) => {
                self.panic_if_outside_root(to);
                self.os_impl.copy_dir(from, to)
            }
        }?;

        Ok(())
    }

    fn check_to_and_from(&self, from: &Path, to: &Path) {
        self.panic_if_outside_root(from);
        self.panic_if_outside_root(to);
    }

    fn panic_if_outside_root(&self, path: &Path) {
        use path_absolutize::*;
        let root = self.resolver().root();
        path.absolutize_virtually(root).unwrap_or_else(|_| {
            panic!(
                "Path {:?} is outside of temp folder root {:?}.\n This is not allowed during development",
                path, root
            );
        });
    }

    pub fn set_cwd(&mut self, cwd: PathBuf) {
        self.cwd = cwd;
    }
}

impl FileManipulator for DevOsFileManipulator {
    type Resolver = DevPathResolver;

    fn copy_file(&self, from: &Path, to: &Path) -> AppIoResult {
        self.check_to_and_from(from, to);
        self.os_impl.copy_file(from, to)
    }

    fn copy_dir(&self, from: &Path, to: &Path) -> AppIoResult {
        self.check_to_and_from(from, to);
        self.os_impl.copy_dir(from, to)
    }

    fn ensure_dir(&self, location: &Path) -> AppIoResult {
        self.panic_if_outside_root(location);
        self.os_impl.ensure_dir(location)
    }

    fn try_exits(&self, location: &Path) -> AppIoResult<bool> {
        self.panic_if_outside_root(location);
        self.os_impl.try_exits(location)
    }

    fn list_first_level_dir(&self, location: &Path) -> AppIoResult<Vec<PathBuf>> {
        self.panic_if_outside_root(location);
        self.os_impl.list_first_level_dir(location)
    }

    fn all_nodes_inside(&self, location: &Path) -> AppIoResult<Vec<NodeEntryMeta>> {
        self.panic_if_outside_root(location);
        self.os_impl.all_nodes_inside(location)
    }

    fn delete_whole_folder(&self, location: &Path) -> AppIoResult {
        self.panic_if_outside_root(location);
        self.os_impl.delete_whole_folder(location)
    }

    fn write_file_to(&self, location: &Path, content: &str) -> AppIoResult {
        self.panic_if_outside_root(location);
        self.os_impl.write_file_to(location, content)
    }

    fn resolver(&self) -> &Self::Resolver {
        &self.resolver
    }

    fn cwd(&self) -> AppIoResult<PathBuf> {
        Ok(self.cwd.to_path_buf())
    }
    fn read_file(&self, path: &Path) -> AppIoResult<String> {
        self.panic_if_outside_root(path);
        self.os_impl.read_file(path)
    }

    fn read_bytes(&self, path: &Path) -> AppIoResult<Vec<u8>> {
        self.panic_if_outside_root(path);
        self.os_impl.read_bytes(path)
    }

    fn write_bytes(&self, path: &Path, to_write: &[u8]) -> AppIoResult {
        self.panic_if_outside_root(path);
        self.os_impl.write_bytes(path, to_write)
    }
}
