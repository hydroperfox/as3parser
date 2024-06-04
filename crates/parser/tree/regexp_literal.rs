use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegExpLiteral {
    pub location: Location,
    pub body: String,
    pub flags: String,
}