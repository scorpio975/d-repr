use serde::{Deserialize, Serialize};
use crate::index::Index;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IndexExpr {
    pub val: Index
}