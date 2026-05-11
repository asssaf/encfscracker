use std::path::Path;
use sled::{Db, Tree};
use once_cell::sync::OnceCell;

static INSTANCE: OnceCell<SledDb> = OnceCell::new();

pub const TREE_TRIED_COMBINATIONS: &str = "tried_combinations";
pub const TREE_PROGRESS: &str = "progress";
pub const KEY_CURRENT_CHECKPOINT: &str = "current_checkpoint";

pub struct SledDb {
    pub db: Db,
}

impl SledDb {
    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn init<P: AsRef<Path>>(path: P) -> anyhow::Result<&'static Self> {
        INSTANCE.get_or_try_init(|| Self::open(path))
    }

    pub fn get() -> Option<&'static Self> {
        INSTANCE.get()
    }

    pub fn tried_tree(&self) -> anyhow::Result<Tree> {
        Ok(self.db.open_tree(TREE_TRIED_COMBINATIONS)?)
    }

    pub fn progress_tree(&self) -> anyhow::Result<Tree> {
        Ok(self.db.open_tree(TREE_PROGRESS)?)
    }

    pub fn mark_as_tried(&self, combination: &[&str]) -> anyhow::Result<()> {
        let tree = self.tried_tree()?;
        let key = combination.join("\0");
        tree.insert(key, &[])?;
        Ok(())
    }

    pub fn is_tried(&self, combination: &[&str]) -> anyhow::Result<bool> {
        let tree = self.tried_tree()?;
        let key = combination.join("\0");
        Ok(tree.contains_key(key)?)
    }

    pub fn save_checkpoint(&self, value: &str) -> anyhow::Result<()> {
        let tree = self.progress_tree()?;
        tree.insert(KEY_CURRENT_CHECKPOINT, value)?;
        Ok(())
    }

    pub fn load_checkpoint(&self) -> anyhow::Result<Option<String>> {
        let tree = self.progress_tree()?;
        let res = tree.get(KEY_CURRENT_CHECKPOINT)?;
        match res {
            Some(ivec) => Ok(Some(String::from_utf8(ivec.to_vec())?)),
            None => Ok(None),
        }
    }

    pub fn reset_state(&self) -> anyhow::Result<()> {
        self.tried_tree()?.clear()?;
        self.progress_tree()?.clear()?;
        Ok(())
    }
}

impl Clone for SledDb {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
        }
    }
}
