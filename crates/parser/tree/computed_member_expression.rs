use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedMemberExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    /// ASDoc. Always ignore this field; it is used solely
    /// when parsing meta-data.
    pub asdoc: Option<Rc<Asdoc>>,
    pub key: Rc<Expression>,
}