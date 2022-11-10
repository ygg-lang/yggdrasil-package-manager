use std::{marker::PhantomData, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use serde_binary::{binary_stream::Endian, from_slice, to_vec};
use sled::{Config, Db, IVec};

use crate::{DictError, Result};

mod iter;

/// A map store on disk
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
    /// Create a new dict on disk
    pub fn new(path: &Path) -> Result<Self> {
        let compression = cfg!(feature = "compression");
        let database = Config::default().use_compression(compression).path(path).open()?;
        Ok(Self { database, typed: Default::default() })
    }
    /// Check if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.database.is_empty()
    }
    /// Returns the on-disk size of the storage files for this database.
    pub fn size(&self) -> Result<u64> {
        Ok(self.database.size_on_disk()?)
    }
    /// Get the value by key name, return `None` if no such key
    #[inline]
    pub fn get(&self, key: K) -> Option<V> {
        self.try_get(key).ok()
    }
    pub fn try_get(&self, key: K) -> Result<V> {
        let k = key.as_ref();
        match self.database.get(k)? {
            Some(iv) => cast_iv(iv),
            None => Err(DictError::KeyNotFound),
        }
    }
    /// Insert the value by key name, return `None` if no such key
    #[inline]
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.try_insert(key, value).ok()
    }
    /// Trying to insert the value by key name, return `None` if no such key
    pub fn try_insert(&self, key: K, value: V) -> Result<V> {
        let k = key.as_ref();
        let v = to_vec(&value, Endian::Little)?;
        match self.database.insert(k, v.clone())? {
            Some(iv) => cast_iv(iv),
            None => Err(DictError::KeyNotFound),
        }
    }
    /// Asynchronously flushes all dirty IO buffers and calls fsync. If this succeeds, it is guaranteed that all previous writes will be recovered if the system crashes. Returns the number of bytes flushed during this call.
    ///
    /// Flushing can take quite a lot of time, and you should measure the performance impact of using it on realistic sustained workloads running on realistic hardware.
    pub async fn flush(&self) -> Result<usize> {
        Ok(self.database.flush_async().await?)
    }
}

fn cast_iv<T>(s: IVec) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(from_slice(s.as_ref(), Endian::Little)?)
}
