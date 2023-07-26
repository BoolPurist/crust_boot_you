use std::{cell::RefCell, collections::HashSet};

use crate::template_augmentation::{AugementKey, FakeAugmentStore, KeyExtact, TestAugmentStore};

use super::ConsoleFetcher;

pub struct TestConsoleFetcher {
    store: RefCell<FakeAugmentStore>,
    accessed_keys: RefCell<HashSet<AugementKey>>,
}

impl ConsoleFetcher for TestConsoleFetcher {
    fn fetch_from(&self, key: KeyExtact) -> crate::template_augmentation::OptAugmentationResult {
        let mut mut_store = self.store.borrow_mut();
        let mut mut_founds = self.accessed_keys.borrow_mut();
        if !mut_founds.insert(key.to_string()) {
            panic!("Key: {} already accessed", key);
        }

        match mut_store.get_mut(key) {
            None => Ok(None),
            Some(value) => Ok(Some(value.clone())),
        }
    }
}

impl TestConsoleFetcher {
    pub fn new(store: TestAugmentStore) -> Self {
        Self {
            store: RefCell::new(store),
            accessed_keys: Default::default(),
        }
    }
}
