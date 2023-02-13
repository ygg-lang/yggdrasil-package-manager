use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_binary::{binary_stream::Endian, from_slice, to_vec};
use sled::{Config, Db, IVec};
use uuid::Uuid;
use yggdrasil_error::YggdrasilResult;

pub struct PackageObjectManager {
    database: PersistenceMap<Uuid, PackageObject>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackageObject {}

pub struct PersistenceMap<K, V> {
    database: Db,
    typed: PhantomData<(K, V)>,
}

impl Drop for PackageObjectManager {
    fn drop(&mut self) {
        match self.database.flush() {
            Ok(e) => {
                print!("FilesManager flush {e} bytes")
            }
            Err(e) => {
                eprintln!("FilesManager drop error. {e}")
            }
        }
    }
}

impl<K, V> PersistenceMap<K, V> {
    pub fn new(dir: &Path, name: &str) -> YggdrasilResult<Self> {
        let path = dir.join(name);
        let database = Config::default().use_compression(true).path(path).open()?;
        Ok(Self { database, typed: Default::default() })
    }
    pub fn get(&self, key: K) -> Option<V> {
        let value = self.database.get(key).ok()??;
        Self::from_iv(value)
    }
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        let value = to_vec(&value, Endian::Little).ok()?;
        let value = self.database.insert(key, value).ok()??;
        Self::from_iv(value)
    }
    pub async fn flush(&self) -> YggdrasilResult<usize> {
        Ok(self.database.flush_async().await?)
    }
    fn from_iv(s: IVec) -> Option<V> {
        from_slice(s.as_ref(), Endian::Little).ok()
    }
}

impl PackageObjectManager {
    pub fn new(dir: &Path) -> YggdrasilResult<Self> {
        let path = dir.join("files");
        let db = Config::default().use_compression(true).path(path).open()?;
        Ok(Self { database: db, phantom_dict: Default::default() })
    }
    pub fn get(&self, key: Uuid) -> Option<PackageObject> {
        let value = self.database.get(key).ok()??;
        Self::from_iv(value)
    }
    pub fn insert(&self, key: Uuid, value: PackageObject) -> Option<PackageObject> {
        let value = to_vec(&value, Endian::Little).ok()?;
        let value = self.database.insert(key, value).ok()??;
        Self::from_iv(value)
    }
    pub async fn flush(&self) -> YggdrasilResult<usize> {
        Ok(self.database.flush_async().await?)
    }
    fn from_iv(s: IVec) -> Option<PackageObject> {
        from_slice(s.as_ref(), Endian::Little).ok()
    }
}

#[tokio::test]
async fn test_files() {
    let path = PathBuf::from("sqlite");
    let file_db = PackageObjectManager::new(&path).unwrap();
    let key = Uuid::new_v4();
    let value = PackageObject {};
    file_db.insert(key, value);
    println!("{:?}", file_db.get(key));

    // file_db.test().await.unwrap()
}
