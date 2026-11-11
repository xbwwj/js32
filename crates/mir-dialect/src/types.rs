use pliron::derive::pliron_type;

#[pliron_type(name = "mir.tuple", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MirTupleType {}

#[pliron_type(name = "mir.ptr", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MirPtrType {}

#[pliron_type(name = "mir.slice", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MirSliceType {}

#[pliron_type(name = "mir.struct", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MirStructType {}

#[pliron_type(name = "mir.unoin", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MirUnionType {}

#[pliron_type(name = "mir.array", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MirArrayType {}

#[pliron_type(name = "mir.enum", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MirEnumType {}
