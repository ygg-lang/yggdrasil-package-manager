use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde_binary::{binary_stream::Endian, from_slice, to_vec};
use sled::{Config, Db, IVec};
use uuid::Uuid;

pub struct DiskMap<K, V> {
    database: Db,
    typed: PhantomData<(K, V)>,
}

impl<K, V> Drop for DiskMap<K, V> {
    fn drop(&mut self) {
        self.database.flush().ok();
    }
}

impl<K, V> DiskMap<K, V> {
    pub fn new(path: &Path) -> Result<Self, sled::Error> {
        let database = Config::default().use_compression(true).path(path).open()?;
        Ok(Self { database, typed: Default::default() })
    }
    pub fn get(&self, key: K) -> Option<V> {
        let value = self.database.get(key).ok()??;
        cast_iv(value)
    }
    pub fn insert(&self, key: K, value: V) -> Result<V, sled::Error> {
        let v = to_vec(&value, Endian::Little).ok()?;
        let iv = self.database.insert(key, v).ok()??;
        cast_iv(iv)
    }
    pub async fn flush(&self) -> Result<usize, sled::Error> {
        Ok(self.database.flush_async().await?)
    }
}

fn cast_iv(s: IVec) -> Option<V> {
    from_slice(s.as_ref(), Endian::Little).ok()
}

#[tokio::test]
async fn test_files() {
    let path = PathBuf::from("sqlite");
    let file_db = DiskMap::new(&path).unwrap();
    let key = Uuid::new_v4();
    let value = PackageObject {};
    file_db.insert(key, value);
    println!("{:?}", file_db.get(key));

    // file_db.test().await.unwrap()
}
