use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asdoc {
    pub location: Location,
    pub main_body: Option<(String, Location)>,
    pub tags: Vec<(AsdocTag, Location)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AsdocTag {
    Author(String),
    Copy(Rc<AsdocReference>),
    Created(String),
    Default(String),
    Deprecated {
        message: Option<String>,
    },
    EventType(Rc<Expression>),
    Example(String),
    InheritDoc,
    Internal(String),
    Langversion(String),
    Param {
        name: String,
        description: String,
    },
    Playerversion(String),
    Private,
    Productversion(String),
    Return(String),
    See {
        reference: Rc<AsdocReference>,
        display_text: Option<String>,
    },
    Throws {
        class_reference: Rc<Expression>,
        description: Option<String>,
    },
    Version(String),
}

/// An ASDoc reference consisting of an optional base and
/// an optional instance property fragment (`#x`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsdocReference {
    /// Base expression.
    pub base: Option<Rc<Expression>>,
    /// Instance property fragment following the hash character.
    pub instance_property: Option<Rc<QualifiedIdentifier>>,
}