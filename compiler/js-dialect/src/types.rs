use pliron::{
    common_traits::Verify,
    context::Context,
    derive::{CloneIntoContext, format, pliron_type},
    identifier::Identifier,
    location::Location,
    r#type::TypeHandle,
    verify_err,
};

macro_rules! js_primitive_type {
    ($jsty:tt, $mnem:literal) => {
        #[pliron_type(name = $mnem, format, verifier = "succ")]
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $jsty;
    };
}

js_primitive_type!(JsUndefinedType, "js.undefined");
js_primitive_type!(JsNullType, "js.null");
js_primitive_type!(JsBooleanType, "js.boolean");
js_primitive_type!(JsNumberType, "js.number");
js_primitive_type!(JsBigIntType, "js.bigint");
js_primitive_type!(JsStringType, "js.string");
js_primitive_type!(JsSymbolType, "js.symbol");
js_primitive_type!(JsObjectType, "js.object");
js_primitive_type!(JsFunctionType, "js.function");

js_primitive_type!(JsAnyType, "js.any");
js_primitive_type!(JsUnknownType, "js.unknown");
js_primitive_type!(JsNeverType, "js.never");
js_primitive_type!(JsVoidType, "js.void");

#[pliron_type(name = "js.array", format = "$element `[]`", verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsArrayType {
    pub element: TypeHandle,
}

#[format("`, ...` $0")]
#[derive(Clone, Debug, Eq, Hash, PartialEq, CloneIntoContext)]
pub struct JsRestParam(TypeHandle);

impl From<TypeHandle> for JsRestParam {
    fn from(ty: TypeHandle) -> Self {
        Self(ty)
    }
}

/// The typed js function type, compared to untyped `js.function`.
#[pliron_type(
    name = "js.fn",
    format = "`(` vec($params, CharSpace(`,`)) opt($rest) `) => ` $ret",
    verifier = "succ"
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsFnType {
    pub params: Vec<TypeHandle>,
    pub rest: Option<JsRestParam>,
    pub ret: TypeHandle,
}

/// Tuple type in typescript, like `number | bigint`.
///
/// We do not verify unification here, as it does not belong to representation level.
#[pliron_type(
    name = "js.tuple",
    format = "`[` vec($types, CharSpace(`,`)) `]`",
    verifier = "succ"
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsTupleType {
    pub types: Vec<TypeHandle>,
}

#[pliron_type(name = "js.union", format = "`<` vec($types, CharSpace(`,`)) `>`")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsUnionType {
    pub types: Vec<TypeHandle>,
}

impl Verify for JsUnionType {
    fn verify(&self, _ctx: &Context) -> pliron::result::Result<()> {
        if self.types.len() < 2 {
            return verify_err!(
                Location::Unknown,
                "JsUnionType must contain at least two types"
            );
        }
        Ok(())
    }
}

#[format("$name ` : ` $ty")]
#[derive(Clone, Debug, Eq, Hash, PartialEq, CloneIntoContext)]
pub struct JsRecordField {
    // TODO: we should further allow symbol, keyed, spread, optional, etc.
    pub name: Identifier,
    pub ty: TypeHandle,
}

#[pliron_type(
    name = "js.record",
    format = "`<` `{` vec($fields, CharSpace(`,`)) `}` `>`"
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, CloneIntoContext)]
pub struct JsRecordType {
    pub fields: Vec<JsRecordField>,
}

impl JsRecordType {
    pub fn new(mut fields: Vec<JsRecordField>) -> Self {
        fields.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        Self { fields }
    }
}

impl Verify for JsRecordType {
    fn verify(&self, ctx: &Context) -> pliron::result::Result<()> {
        // Fields must be sorted and unique.
        for pair in self.fields.windows(2) {
            if pair[0].name.cmp(&pair[1].name) != core::cmp::Ordering::Less {
                return verify_err!(
                    Location::Unknown,
                    "js.record fields must be sorted and unique"
                );
            }
        }

        for field in &self.fields {
            field.ty.verify(ctx)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pliron::{context::Context, printable::Printable, r#type::Type};

    // This test is used to check the generated mlir is as simple
    // as we want, otherwise we need to change to `format`.
    #[test]
    fn test_primitive_type_serialization() {
        let ctx = Context::new();

        macro_rules! case {
            ($jsty:tt, $s:literal) => {{
                // Serialize
                let ty = $jsty::instantiate($jsty, &ctx).to_handle();
                let serialized = ty.disp(&ctx).to_string();
                assert_eq!(serialized.trim(), $s);

                // TODO: deserialize
            }};
        }

        case!(JsAnyType, "js.any");
        case!(JsUndefinedType, "js.undefined");
        case!(JsNullType, "js.null");
        case!(JsBooleanType, "js.boolean");
        case!(JsNumberType, "js.number");
        case!(JsBigIntType, "js.bigint");
        case!(JsStringType, "js.string");
        case!(JsSymbolType, "js.symbol");
        case!(JsObjectType, "js.object");
        case!(JsFunctionType, "js.function");
        case!(JsAnyType, "js.any");
        case!(JsUnknownType, "js.unknown");
        case!(JsNeverType, "js.never");
        case!(JsVoidType, "js.void");
    }

    #[test]
    fn test_fn_type_serialization() {
        let ctx = Context::new();

        let number_ty = JsNumberType::instantiate(JsNumberType {}, &ctx).to_handle();
        let string_ty = JsStringType::instantiate(JsStringType {}, &ctx).to_handle();

        let ty = JsFnType::instantiate(
            JsFnType {
                params: vec![string_ty.into(), number_ty.into()],
                rest: Some(string_ty.into()),
                ret: string_ty,
            },
            &ctx,
        )
        .to_handle();

        let serialized = ty.disp(&ctx).to_string();
        assert_eq!(
            serialized.trim(),
            "js.fn (js.string , js.number , ...js.string ) => js.string"
        );
    }

    #[test]
    fn test_tuple_type_serialization() {
        let ctx = Context::new();

        let number_ty = JsNumberType::instantiate(JsNumberType {}, &ctx).to_handle();
        let string_ty = JsStringType::instantiate(JsStringType {}, &ctx).to_handle();

        let ty = JsTupleType::instantiate(
            JsTupleType {
                types: vec![number_ty, string_ty],
            },
            &ctx,
        )
        .to_handle();

        let serialized = ty.disp(&ctx).to_string();
        assert_eq!(serialized.trim(), "js.tuple [js.number , js.string ]");
    }

    #[test]
    fn test_union_type_serialization() {
        let ctx = Context::new();

        let number_ty = JsNumberType::instantiate(JsNumberType {}, &ctx).to_handle();
        let string_ty = JsStringType::instantiate(JsStringType {}, &ctx).to_handle();

        let ty = JsUnionType::instantiate(
            JsUnionType {
                types: vec![number_ty, string_ty],
            },
            &ctx,
        )
        .to_handle();

        let serialized = ty.disp(&ctx).to_string();
        // TODO: union seem to have extra space here. tolerate for now.
        assert_eq!(serialized.trim(), "js.union <js.number , js.string >");
    }

    #[test]
    fn test_record_type_serialization() {
        let ctx = Context::new();

        let number_ty = JsNumberType::instantiate(JsNumberType {}, &ctx).to_handle();
        let string_ty = JsStringType::instantiate(JsStringType {}, &ctx).to_handle();

        let ty = JsRecordType::instantiate(
            JsRecordType {
                fields: vec![
                    JsRecordField {
                        name: pliron::ident!("name"),
                        ty: string_ty,
                    },
                    JsRecordField {
                        name: pliron::ident!("age"),
                        ty: number_ty,
                    },
                ],
            },
            &ctx,
        )
        .to_handle();

        let serialized = ty.disp(&ctx).to_string();

        assert_eq!(
            serialized.trim(),
            "js.record <{name : js.string , age : js.number }>"
        );
    }
}
