use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde::Serialize;
use serde_binary::{binary_stream::Endian, from_slice, to_vec};
use sled::{Config, Db, IVec};

use crate::{DictError::KeyNotFound, DictResult};

pub struct DiskMap<K, V> {
    database: Db,
    typed: PhantomData<(K, V)>,
}

impl<K, V> Drop for DiskMap<K, V> {
    fn drop(&mut self) {
        self.database.flush().ok();
    }
}

impl<K, V> DiskMap<K, V>
where
    K: AsRef<[u8]>,
    V: Serialize,
{
    pub fn new(path: &Path) -> DictResult<Self> {
        let database = Config::default().use_compression(true).path(path).open()?;
        Ok(Self { database, typed: Default::default() })
    }
    pub fn get(&self, key: K) -> DictResult<V> {
        let k = key.as_ref();
        match self.database.get(k)? {
            Some(iv) => cast_iv(iv),
            None => Err(KeyNotFound(k.to_vec())),
        }
    }
    pub fn insert(&self, key: K, value: V) -> DictResult<V> {
        let v = to_vec(&value, Endian::Little)?;
        match self.database.insert(key, &v)? {
            Some(iv) => cast_iv(iv),
            None => Err(KeyNotFound(v)),
        }
    }
    pub async fn flush(&self) -> DictResult<usize> {
        Ok(self.database.flush_async().await?)
    }
}

fn cast_iv<V>(s: IVec) -> DictResult<V> {
    Ok(from_slice(s.as_ref(), Endian::Little)?)
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
