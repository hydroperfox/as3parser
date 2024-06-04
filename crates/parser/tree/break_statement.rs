use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakStatement {
    pub location: Location,
    pub label: Option<(String, Location)>,
}