use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};
use serde_binary::{binary_stream::Endian, from_slice, to_vec};
use sled::{Config, Db, IVec};

use crate::{DictError, DictResult};

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
    V: Serialize + DeserializeOwned,
{
    pub fn new(path: &Path) -> DictResult<Self> {
        let database = Config::default().use_compression(true).path(path).open()?;
        Ok(Self { database, typed: Default::default() })
    }
    pub fn get(&self, key: K) -> DictResult<V> {
        let k = key.as_ref();
        match self.database.get(k)? {
            Some(iv) => cast_iv(iv),
            None => Err(DictError::KeyNotFound(k.to_vec())),
        }
    }
    pub fn insert(&self, key: K, value: V) -> DictResult<V> {
        let k = key.as_ref();
        let v = to_vec(&value, Endian::Little)?;
        match self.database.insert(k, v.clone())? {
            Some(iv) => cast_iv(iv),
            None => Err(DictError::KeyNotFound(k.to_vec())),
        }
    }
    pub async fn flush(&self) -> DictResult<usize> {
        Ok(self.database.flush_async().await?)
    }
}

fn cast_iv<T>(s: IVec) -> DictResult<T>
where
    T: DeserializeOwned,
{
    Ok(from_slice(s.as_ref(), Endian::Little)?)
}

#[tokio::test]
async fn test_files() {
    let path = PathBuf::from("sqlite");
    let file_db = DiskMap::new(&path).unwrap();
    let key = "key";
    let value = "value".to_string();
    file_db.insert(key, value);
    println!("{:?}", file_db.get("key").unwrap());

    // file_db.test().await.unwrap()
}
