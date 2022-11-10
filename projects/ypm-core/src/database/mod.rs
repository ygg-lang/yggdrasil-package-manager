use std::path::{Path, PathBuf};

use sled::{Config, Db};
use yggdrasil_error::YggdrasilResult;

pub struct FileID(u128);

pub struct FilesManager {
    database: Db,
}

impl FilesManager {
    pub fn new(dir: &Path) -> YggdrasilResult<Self> {
        let path = dir.join("files");
        let db = Config::default().use_compression(true).path(path).open()?;
        Ok(Self {
            database: db
        })
    }
}


impl FilesManager {
    async fn test(&self) -> YggdrasilResult {
        let tree = &self.database;

// insert and get, similar to std's BTreeMap
        let old_value = tree.insert("key", "value")?;

        assert_eq!(
            tree.get(&"key")?,
            Some(sled::IVec::from("value")),
        );


        for kv_result in tree.range("key_1".."key_9") {}
        let old_value = tree.remove(&"key")?;
        tree.compare_and_swap(
            "key",
            Some("current_value"),
            Some("new_value"),
        )??;
        tree.flush_async().await?;
        Ok(())
    }
}

#[tokio::test]
async fn test() {
    let path = PathBuf::from("sqlite");
    let file = FilesManager::new(&path).unwrap();
    file.test().await.unwrap()
}