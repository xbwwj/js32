use pliron::{
    common_traits::Verify, context::Context, derive::pliron_type, location::Location,
    r#type::TypeHandle, verify_err,
};

#[pliron_type(name = "js.any", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsAnyType {}

#[pliron_type(name = "js.undefined", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsUndefinedType {}

#[pliron_type(name = "js.null", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsNullType {}

#[pliron_type(name = "js.boolean", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsBooleanType {}

#[pliron_type(name = "js.number", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsNumberType {}

#[pliron_type(name = "js.bigint", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsBigIntType {}

#[pliron_type(name = "js.string", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsStringType {}

#[pliron_type(name = "js.symbol", format, verifier = "succ")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct JsSymbolType {}

// TODO: formalization of union type.
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
                let ty = $jsty::instantiate($jsty {}, &ctx).to_handle();
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
}
