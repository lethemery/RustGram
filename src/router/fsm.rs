//! Finite State Machine (FSM) traits and in-memory storage for multi-step bot conversations.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use tokio::sync::RwLock;

use crate::error::{Error, Result};

/// Composite key identifying an FSM conversation slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateKey {
    /// Chat identifier.
    pub chat_id: i64,
    /// Acting user identifier (`0` if unavailable, e.g. channel post).
    pub user_id: i64,
}

impl StateKey {
    /// Creates a new [`StateKey`].
    #[inline]
    pub const fn new(chat_id: i64, user_id: i64) -> Self {
        Self { chat_id, user_id }
    }
}

/// Boxed future helper for object-safe async storage traits.
pub type StorageFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

/// Asynchronous key-value state and context-data storage backend for FSM bots.
pub trait StateStorage<S>: Send + Sync
where
    S: Clone + Send + Sync + 'static,
{
    /// Retrieves the current FSM state for `key`.
    fn get_state(&self, key: StateKey) -> StorageFuture<'_, Option<S>>;

    /// Sets the FSM state for `key`.
    fn set_state(&self, key: StateKey, state: S) -> StorageFuture<'_, ()>;

    /// Clears both the FSM state and associated data map for `key`.
    fn clear(&self, key: StateKey) -> StorageFuture<'_, ()>;

    /// Clears only the FSM state for `key` while keeping stored data.
    fn clear_state(&self, key: StateKey) -> StorageFuture<'_, ()>;

    /// Retrieves the JSON data map associated with `key`.
    fn get_data(&self, key: StateKey) -> StorageFuture<'_, Map<String, Value>>;

    /// Replaces the JSON data map associated with `key`.
    fn set_data(&self, key: StateKey, data: Map<String, Value>) -> StorageFuture<'_, ()>;
}

#[derive(Debug, Clone)]
struct StateEntry<S> {
    state: Option<S>,
    data: Map<String, Value>,
}

impl<S> Default for StateEntry<S> {
    fn default() -> Self {
        Self {
            state: None,
            data: Map::new(),
        }
    }
}

/// Thread-safe in-memory [`StateStorage`] implementation suitable for single-instance bots and tests.
#[derive(Debug)]
pub struct MemoryStorage<S> {
    inner: RwLock<HashMap<StateKey, StateEntry<S>>>,
}

impl<S> Default for MemoryStorage<S> {
    fn default() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }
}

impl<S> MemoryStorage<S> {
    /// Creates a new empty [`MemoryStorage`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S> StateStorage<S> for MemoryStorage<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn get_state(&self, key: StateKey) -> StorageFuture<'_, Option<S>> {
        Box::pin(async move {
            let guard = self.inner.read().await;
            Ok(guard.get(&key).and_then(|e| e.state.clone()))
        })
    }

    fn set_state(&self, key: StateKey, state: S) -> StorageFuture<'_, ()> {
        Box::pin(async move {
            let mut guard = self.inner.write().await;
            guard.entry(key).or_default().state = Some(state);
            Ok(())
        })
    }

    fn clear(&self, key: StateKey) -> StorageFuture<'_, ()> {
        Box::pin(async move {
            let mut guard = self.inner.write().await;
            guard.remove(&key);
            Ok(())
        })
    }

    fn clear_state(&self, key: StateKey) -> StorageFuture<'_, ()> {
        Box::pin(async move {
            let mut guard = self.inner.write().await;
            if let Some(entry) = guard.get_mut(&key) {
                entry.state = None;
            }
            Ok(())
        })
    }

    fn get_data(&self, key: StateKey) -> StorageFuture<'_, Map<String, Value>> {
        Box::pin(async move {
            let guard = self.inner.read().await;
            Ok(guard.get(&key).map(|e| e.data.clone()).unwrap_or_default())
        })
    }

    fn set_data(&self, key: StateKey, data: Map<String, Value>) -> StorageFuture<'_, ()> {
        Box::pin(async move {
            let mut guard = self.inner.write().await;
            guard.entry(key).or_default().data = data;
            Ok(())
        })
    }
}

/// Ergonomic handle to a single user/chat FSM slot.
#[derive(Clone)]
pub struct StateContext<S>
where
    S: Clone + Send + Sync + 'static,
{
    key: StateKey,
    storage: Arc<dyn StateStorage<S>>,
}

impl<S> StateContext<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Creates a new [`StateContext`] for `key` backed by `storage`.
    pub fn new(key: StateKey, storage: Arc<dyn StateStorage<S>>) -> Self {
        Self { key, storage }
    }

    /// Returns the underlying [`StateKey`].
    #[inline]
    pub const fn key(&self) -> StateKey {
        self.key
    }

    /// Gets the current state.
    pub async fn get(&self) -> Result<Option<S>> {
        self.storage.get_state(self.key).await
    }

    /// Sets the current state.
    pub async fn set(&self, state: S) -> Result<()> {
        self.storage.set_state(self.key, state).await
    }

    /// Clears both state and data for this slot.
    pub async fn clear(&self) -> Result<()> {
        self.storage.clear(self.key).await
    }

    /// Clears only the state, keeping stored key-value data intact.
    pub async fn clear_state(&self) -> Result<()> {
        self.storage.clear_state(self.key).await
    }

    /// Inserts or updates a typed field inside the slot's data map.
    pub async fn insert<V: Serialize>(&self, field: impl Into<String>, value: V) -> Result<()> {
        let mut data = self.storage.get_data(self.key).await?;
        let encoded = serde_json::to_value(value).map_err(Error::Json)?;
        data.insert(field.into(), encoded);
        self.storage.set_data(self.key, data).await
    }

    /// Reads and deserializes a typed field from the slot's data map.
    pub async fn get_value<V: DeserializeOwned>(&self, field: &str) -> Result<Option<V>> {
        let data = self.storage.get_data(self.key).await?;
        match data.get(field) {
            Some(val) => serde_json::from_value(val.clone())
                .map(Some)
                .map_err(Error::Json),
            None => Ok(None),
        }
    }

    /// Returns the raw JSON data map for this slot.
    pub async fn data(&self) -> Result<Map<String, Value>> {
        self.storage.get_data(self.key).await
    }
}
