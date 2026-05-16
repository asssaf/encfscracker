use once_cell::sync::OnceCell;
use sled::{Db, Tree};
use std::path::Path;

static INSTANCE: OnceCell<SledDb> = OnceCell::new();

pub const TREE_TRIED_COMBINATIONS: &str = "tried_combinations";
pub const TREE_PROGRESS: &str = "progress";
pub const TREE_FRAGMENTS: &str = "fragments";
pub const TREE_GROUPS: &str = "groups";
pub const TREE_CONFIG: &str = "config";
pub const KEY_CURRENT_CHECKPOINT: &str = "current_checkpoint";
const KEY_SALT: &str = "salt";
const KEY_CANARY: &str = "canary";
const CANARY_VALUE: &[u8] = b"encfs_cracker_v1";

pub struct SledDb {
    pub db: Db,
    master_key: OnceCell<[u8; 32]>,
}

impl SledDb {
    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let db = sled::open(path)?;
        Ok(Self {
            db,
            master_key: OnceCell::new(),
        })
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

    pub fn fragments_tree(&self) -> anyhow::Result<Tree> {
        Ok(self.db.open_tree(TREE_FRAGMENTS)?)
    }

    pub fn groups_tree(&self) -> anyhow::Result<Tree> {
        Ok(self.db.open_tree(TREE_GROUPS)?)
    }

    pub fn config_tree(&self) -> anyhow::Result<Tree> {
        Ok(self.db.open_tree(TREE_CONFIG)?)
    }

    /// Checks if the database is already initialized with encryption.
    pub fn needs_initialization(&self) -> anyhow::Result<bool> {
        let config = self.config_tree()?;
        Ok(!config.contains_key(KEY_SALT)?)
    }

    /// Initializes a new database with encryption.
    pub fn initialize_encryption(&self, password: &str) -> anyhow::Result<()> {
        use crate::crypto::state_encryption;
        use rand::{rng, RngExt};

        let mut salt = [0u8; 16];
        rng().fill(&mut salt);

        let key = state_encryption::derive_key(password, &salt);

        let config = self.config_tree()?;
        config.insert(KEY_SALT, &salt)?;

        // Save an encrypted canary to verify the password later
        let encrypted_canary = state_encryption::encrypt(CANARY_VALUE, &key);
        config.insert(KEY_CANARY, encrypted_canary)?;

        self.master_key
            .set(key)
            .map_err(|_| anyhow::anyhow!("Master key already set"))?;
        Ok(())
    }

    /// Unlocks an existing database with the password.
    pub fn unlock(&self, password: &str) -> anyhow::Result<()> {
        use crate::crypto::state_encryption;

        let config = self.config_tree()?;
        let salt = config
            .get(KEY_SALT)?
            .ok_or_else(|| anyhow::anyhow!("Database not initialized (salt missing)"))?;

        let key = state_encryption::derive_key(password, &salt);

        let encrypted_canary = config
            .get(KEY_CANARY)?
            .ok_or_else(|| anyhow::anyhow!("Database corrupted (canary missing)"))?;

        // Attempt to decrypt canary to verify password
        state_encryption::decrypt(&encrypted_canary, &key)
            .map_err(|_| anyhow::anyhow!("Incorrect password"))?;

        self.master_key
            .set(key)
            .map_err(|_| anyhow::anyhow!("Master key already set"))?;
        Ok(())
    }

    fn get_key(&self) -> anyhow::Result<&[u8; 32]> {
        self.master_key
            .get()
            .ok_or_else(|| anyhow::anyhow!("Database is locked. Please unlock first."))
    }

    pub fn add_fragment(&self, fragment: &crate::state::Fragment) -> anyhow::Result<()> {
        use crate::crypto::state_encryption;
        let key = self.get_key()?;
        let tree = self.fragments_tree()?;
        let value = serde_json::to_vec(fragment)?;
        let encrypted_value = state_encryption::encrypt(&value, key);
        tree.insert(&fragment.text, encrypted_value)?;
        Ok(())
    }

    pub fn list_fragments(&self) -> anyhow::Result<Vec<crate::state::Fragment>> {
        use crate::crypto::state_encryption;
        let key = self.get_key()?;
        let tree = self.fragments_tree()?;
        let mut fragments = Vec::new();
        for item in tree.iter() {
            let (_, v) = item?;
            let decrypted_v = state_encryption::decrypt(&v, key)?;
            let fragment: crate::state::Fragment = serde_json::from_slice(&decrypted_v)?;
            fragments.push(fragment);
        }
        Ok(fragments)
    }

    pub fn clear_fragments(&self) -> anyhow::Result<()> {
        self.fragments_tree()?.clear()?;
        Ok(())
    }

    pub fn add_group(&self, group: &crate::state::FragmentGroup) -> anyhow::Result<()> {
        use crate::crypto::state_encryption;
        let key = self.get_key()?;
        let tree = self.groups_tree()?;
        let value = serde_json::to_vec(group)?;
        let encrypted_value = state_encryption::encrypt(&value, key);
        tree.insert(&group.id, encrypted_value)?;
        Ok(())
    }

    pub fn list_groups(&self) -> anyhow::Result<Vec<crate::state::FragmentGroup>> {
        use crate::crypto::state_encryption;
        let key = self.get_key()?;
        let tree = self.groups_tree()?;
        let mut groups = Vec::new();
        for item in tree.iter() {
            let (_, v) = item?;
            let decrypted_v = state_encryption::decrypt(&v, key)?;
            let group: crate::state::FragmentGroup = serde_json::from_slice(&decrypted_v)?;
            groups.push(group);
        }
        Ok(groups)
    }

    pub fn clear_groups(&self) -> anyhow::Result<()> {
        self.groups_tree()?.clear()?;
        Ok(())
    }

    pub fn mark_as_tried(&self, combination: &[&str]) -> anyhow::Result<()> {
        use sha2::{Digest, Sha256};
        let tree = self.tried_tree()?;
        let raw_key = combination.join("\0");
        let mut hasher = Sha256::new();
        hasher.update(raw_key.as_bytes());
        let hashed_key = hasher.finalize();
        tree.insert(hashed_key, &[])?;
        Ok(())
    }

    pub fn is_tried(&self, combination: &[&str]) -> anyhow::Result<bool> {
        use sha2::{Digest, Sha256};
        let tree = self.tried_tree()?;
        let raw_key = combination.join("\0");
        let mut hasher = Sha256::new();
        hasher.update(raw_key.as_bytes());
        let hashed_key = hasher.finalize();
        Ok(tree.contains_key(hashed_key)?)
    }

    pub fn save_checkpoint(&self, value: &str) -> anyhow::Result<()> {
        use crate::crypto::state_encryption;
        let key = self.get_key()?;
        let tree = self.progress_tree()?;
        let encrypted_value = state_encryption::encrypt(value.as_bytes(), key);
        tree.insert(KEY_CURRENT_CHECKPOINT, encrypted_value)?;
        Ok(())
    }

    pub fn load_checkpoint(&self) -> anyhow::Result<Option<String>> {
        use crate::crypto::state_encryption;
        let key = self.get_key()?;
        let tree = self.progress_tree()?;
        let res = tree.get(KEY_CURRENT_CHECKPOINT)?;
        match res {
            Some(ivec) => {
                let decrypted = state_encryption::decrypt(&ivec, key)?;
                Ok(Some(String::from_utf8(decrypted)?))
            }
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
            master_key: self.master_key.clone(),
        }
    }
}
