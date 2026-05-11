use std::path::Path;
use sled::Db;
use once_cell::sync::OnceCell;

static INSTANCE: OnceCell<SledDb> = OnceCell::new();

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
}

impl Clone for SledDb {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
        }
    }
}
