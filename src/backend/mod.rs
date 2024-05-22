use std::sync::Arc;
use dashmap::DashMap;
use crate::RespFrame;

#[derive(Debug)]
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

impl Backend {
    pub fn new() -> Self {
        Self::default()
    }
}