use std::ops::Deref;
use crate::RespFrame;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug)]
pub struct BackendInner {
    pub(crate) map: DashMap<String, RespFrame>,
    pub(crate) hmap: DashMap<String, DashMap<String, RespFrame>>,
}

impl Default for Backend {
    fn default() -> Self {
        Backend(Arc::new(BackendInner {
            map: DashMap::new(),
            hmap: DashMap::new(),
        }))
    }
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Backend {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }
    
    pub fn set(&self, key: &str, value: RespFrame) {
        self.map.insert(key.to_string(), value);
    }
    
    
    //todo!();
    
}
