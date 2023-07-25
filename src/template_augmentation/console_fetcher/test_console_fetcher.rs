use std::cell::RefCell;

use crate::template_augmentation::{FakeAugmentStore, KeyExtact, TestAugmentStore};

use super::ConsoleFetcher;

pub struct TestConsoleFetcher {
    store: RefCell<FakeAugmentStore>,
}

impl ConsoleFetcher for TestConsoleFetcher {
    fn fetch_from<'a>(
        &self,
        key: KeyExtact<'a>,
    ) -> crate::template_augmentation::OptAugmentationResult {
        let mut mut_store = self.store.borrow_mut();

        match mut_store.get_mut(key) {
            None => Ok(None),
            Some((value, already_accessed)) => {
                if *already_accessed {
                    panic!("Key: {} already accessed", key)
                } else {
                    *already_accessed = true;
                    Ok(value.clone())
                }
            }
        }
    }
}

impl TestConsoleFetcher {
    pub fn new(store: TestAugmentStore) -> Self {
        let store: FakeAugmentStore = store
            .into_iter()
            .map(|(key, value)| (key, (value, false)))
            .collect();
        Self {
            store: RefCell::new(store),
        }
    }
}
