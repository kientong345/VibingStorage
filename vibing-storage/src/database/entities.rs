use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::database::{core::pool::VibingPool, error::Result};

pub mod track;
pub mod vibe;
pub mod vibe_group;

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub total_page: i32,
    pub page_num: i32,
    pub page_size: i32,
}

#[allow(async_fn_in_trait)]
pub trait Paginate<P>: Serialize + Sized {
    async fn page(params: &P, pool: Arc<RwLock<VibingPool>>) -> Result<Page<Self>>;
}
