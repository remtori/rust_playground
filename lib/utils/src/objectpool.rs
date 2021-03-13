use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, Weak},
};

#[derive(Debug)]
pub struct ObjectPool<T> {
    pool: Arc<Mutex<Vec<T>>>,
    object_creator: fn() -> T,
}

impl<T: Default> ObjectPool<T> {
    pub fn new() -> ObjectPool<T> {
        ObjectPool {
            pool: Arc::new(Mutex::new(Vec::new())),
            object_creator: Default::default,
        }
    }
}

impl<T> ObjectPool<T> {
    pub fn with_creator(object_creator: fn() -> T) -> ObjectPool<T> {
        ObjectPool {
            pool: Arc::new(Mutex::new(Vec::new())),
            object_creator,
        }
    }

    pub fn take(&self) -> ObjectPoolGuard<T> {
        let mut pool = self.pool.lock().unwrap();

        if let Some(object) = pool.pop() {
            ObjectPoolGuard::new(object, Arc::downgrade(&self.pool))
        } else {
            ObjectPoolGuard::new((self.object_creator)(), Arc::downgrade(&self.pool))
        }
    }
}

impl<T: Default> Default for ObjectPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ObjectPoolGuard<T> {
    object: Option<T>,
    pool: Weak<Mutex<Vec<T>>>,
}

impl<T> ObjectPoolGuard<T> {
    fn new(object: T, pool: Weak<Mutex<Vec<T>>>) -> ObjectPoolGuard<T> {
        ObjectPoolGuard {
            object: Some(object),
            pool,
        }
    }
}

impl<T> Drop for ObjectPoolGuard<T> {
    fn drop(&mut self) {
        if let Some(obj) = self.object.take() {
            if let Some(guarded_pool) = self.pool.upgrade() {
                if let Ok(mut pool) = guarded_pool.lock() {
                    // This might panic, perhap add a `pool_capacity` field
                    // so that we can get unlimited value from the pool
                    // while only cache `pool_capacity` number of object
                    //
                    // `pool_capcity` can also help to keep the memory in check
                    // in long running application, as of now the pool will cache
                    // the maximum object that ever needed at once
                    // and never deallocate them after
                    pool.push(obj)
                }
            }
        }
    }
}

impl<T> Deref for ObjectPoolGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: The object is only None when we release the Guard so this is safe
        self.object.as_ref().unwrap()
    }
}

impl<T> DerefMut for ObjectPoolGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: The object is only None when we release the Guard so this is safe
        self.object.as_mut().unwrap()
    }
}
