use std::borrow::Cow;

mod dev_path_resolver;
mod os_path_resolver;

pub use dev_path_resolver::DevPathResolver;
pub use os_path_resolver::OsPathResolver;

#[cfg(test)]
mod test_path_resolver;
#[cfg(test)]
pub use test_path_resolver::TestPathResolver;

use crate::prelude::*;

pub trait PathResolver: Default {
    fn root(&self) -> &Path;
    fn try_exits(&self, path: &Path) -> AppIoResult<bool>;

    fn absolute_and_exits<'a>(&self, path: &'a Path) -> AppIoResult<Option<Cow<'a, Path>>> {
        let abs = self.absolute(path)?;
        let exits = self.try_exits(&abs)?;
        let value = if exits { Some(abs) } else { None };
        Ok(value)
    }
    fn absolute<'a>(&self, path: &'a Path) -> AppIoResult<Cow<'a, Path>> {
        use path_absolutize::*;
        let root = self.root();
        let absolute = path.absolutize_virtually(root)?;
        Ok(absolute)
    }
}

#[cfg(test)]
mod testing {
    use crate::app_traits::path_resolver::test_path_resolver::TestPathResolver;

    use super::*;

    #[test]
    fn absolute_some_path() {
        assert_case("/some_root/home", "hello", "/some_root/home/hello");
        assert_case("/", "hello/world", "/hello/world");
        assert_case("/", "", "/");
        assert_case("/", "/../../..", "/");
        assert_case("/x", "/x/world/..", "/x");
        assert_case("/aa", "/aa/hello/../world", "/aa/world");
        assert_case("/aa", "/aa/hello/../z/..", "/aa");
        // assert_case("/ccc/aaa/zzz", "/ccc/aaa/zzz/..", "/ccc/aaa");
        fn assert_case(root: &str, to_resolve: &str, expected: &str) {
            let test_path_resolver = TestPathResolver::new(Path::new(root).to_owned(), true);
            let actual = test_path_resolver.absolute(Path::new(to_resolve)).unwrap();

            let expected = Path::new(expected);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn reject_outside_root() {
        assert_case("/aaa", "/ccccc/world");
        assert_case("/aaa", "/aaa/../world");
        assert_case("/zzzz", "/zzzz/../zz/world");
        assert_case("/zzzz", "/zzzz/zz/../../world");
        // assert_case("/ccc/aaa/zzz", "/ccc/aaa/zzz/..", "/ccc/aaa");
        fn assert_case(root: &str, to_resolve: &str) {
            let test_path_resolver = TestPathResolver::new(Path::new(root).to_owned(), true);
            let _ = test_path_resolver
                .absolute(Path::new(to_resolve))
                .unwrap_err();
        }
    }

    #[test]
    fn success_for_absulute_and_but_does_not_exits() {
        let test_path_resolver = TestPathResolver::new(Path::new("/").to_owned(), false);
        let exits = test_path_resolver
            .absolute_and_exits(Path::new("/"))
            .unwrap();
        assert!(exits.is_none());
    }

    #[test]
    fn success_for_absulute_and_exits() {
        let test_path_resolver = TestPathResolver::new(Path::new("/").to_owned(), true);
        let exits = test_path_resolver
            .absolute_and_exits(Path::new("/"))
            .unwrap();
        assert!(matches!(exits, Some(Cow::Borrowed(..))));
    }
}
