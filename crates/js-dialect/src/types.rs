use pliron::derive::pliron_type;

#[pliron_type(name = "js.any", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsAnyType {}

// TODO: do we need js primitive type defined?
