use pliron::{
    builtin::{
        attributes::{BoolAttr, IdentifierAttr, StringAttr},
        op_interfaces::{
            IsolatedFromAboveInterface, NOpdsInterface, NRegionsInterface, NResultsInterface,
            NoTerminatorInterface, OneRegionInterface, SingleBlockRegionInterface,
        },
    },
    derive::pliron_op,
};

use crate::types::JsAnyType;

/// `js.program` have two regions, which are directives and body.
/// The directive is something like `use strict;`, `use server;`,
/// while body is the ordinary js code.
#[pliron_op(
    name = "js.program",
    interfaces = [
        NRegionsInterface<2>,
        SingleBlockRegionInterface,
        IsolatedFromAboveInterface,
        NOpdsInterface<0>,
        NResultsInterface<0>,
        NoTerminatorInterface,
    ],
    attributes = (
        // currently we only support js, so no source type
        // source_type: StringAttr,
        hashbang: StringAttr,
    ),
    format,
    verifier = "succ"
)]
pub struct JsProgramOp;

#[pliron_op(
    name = "js.directive",
    attributes = (expression: StringAttr),
    format,
    verifier = "succ",
)]
pub struct JsDirectiveOp;

#[pliron_op(
    name = "js.block_statement",
    interfaces = [
        NRegionsInterface<1>,
        SingleBlockRegionInterface,
        NoTerminatorInterface
    ],
    format,
    verifier = "succ"
)]
pub struct JsBlockStatementOp;

#[pliron_op(
    name = "js.break_statement",
    attributes = (
        break_label: IdentifierAttr
    ),
    format,
    verifier = "succ",
)]
pub struct JsBreakStatementOp;

#[pliron_op(
    name = "js.continue_statement",
    attributes = (
        continue_label: IdentifierAttr
    ),
    format,
    verifier = "succ",
)]
pub struct JsContinueStatementOp;

#[pliron_op(name = "js.debugger_statement", format, verifier = "succ")]
pub struct JsDebuggerStatementOp;

#[pliron_op(
    name = "js.do_while_statement",
    operands = (
        test: JsAnyType,
    ),
    interfaces = [
        OneRegionInterface,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    format,
    verifier = "succ",
)]
pub struct JsDoWhileStatementOp;

#[pliron_op(name = "js.empty_statement", format, verifier = "succ")]
pub struct JsEmptyStatementOp;

#[pliron_op(
    name = "js.expression_statement", 
    operands = (
        expresison: JsAnyType
    ),
    format, verifier = "succ"
)]
pub struct JsExpressionStatementOp;

#[pliron_op(
    name = "js.for_in_statement",
    operands = (
        left: JsAnyType,
        right: JsAnyType,
    ),
    interfaces = [
        OneRegionInterface,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    format,
    verifier = "succ"
)]
pub struct JsForInStatementOp;

#[pliron_op(
    name = "js.for_of_statement",
    attributes = (
        is_await: BoolAttr,
    ),
    operands = (
        left: JsAnyType,
        right: JsAnyType,
    ),
    interfaces = [
        // body
        OneRegionInterface,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    format,
    verifier = "succ"
)]
pub struct JsForOfStatementOp;

#[pliron_op(
    name = "js.for_statement",
    interfaces = [
        // TODO: but test seem to be a expression?
        // init, test, update, body
        NRegionsInterface<4>,
    ],
    format,
    verifier = "succ"
)]
pub struct JsForStatementOp;

#[pliron_op(
    name = "js.if_statement",
    operands = (
        test: JsAnyType,
    ),
    interfaces = [
        NRegionsInterface<2>,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    format,
    verifier = "succ",
)]
pub struct JsIfStatementOp;

#[pliron_op(
    name = "js.labeled_statement",
    attributes = (
        label: IdentifierAttr,
    ),
    format,
    verifier = "succ"
)]
pub struct JsLabeledStatementOp;

#[pliron_op(
    name = "js.return_statement",
    operands = (
        argument: JsAnyType,
    ),
    format,
    verifier = "succ",
)]
pub struct JsReturnStatementOp;

#[pliron_op(
    name = "js.switch_statement",
    operands = (
        discriminant: JsAnyType
    ),
    interfaces = [
        OneRegionInterface,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    format,
    verifier = "succ"
)]
pub struct JsSwitchStatementOp;

#[pliron_op(
    name = "js.throw_statement",
    operands = (
        argument: JsAnyType,
    ),
    format,
    verifier = "succ",
)]
pub struct JsThrowStatementOp;

#[pliron_op(
    name = "js.try_statement",
    interfaces = [
        NRegionsInterface<3>,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    format,
    verifier = "succ"
)]
pub struct JsTryStatementOp;

#[pliron_op(
    name = "js.while_statement",
    operands = (
        test: JsAnyType,
    ),
    interfaces = [
        OneRegionInterface,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    format,
    verifier = "succ"
)]
pub struct JsWhileStatementOp;

#[pliron_op(
    name = "js.with_statement",
    operands = (
        object: JsAnyType,
    ),
    interfaces = [
        OneRegionInterface,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    format,
    verifier = "succ"
)]
pub struct JsWithStatementOp;

#[pliron_op(
    name = "js.variable_declaration",
    format,
    interfaces = [
        OneRegionInterface,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    attributes = (
        js_variable_declaration_kind: StringAttr,
        variable_is_declare: BoolAttr,
    ),
    verifier = "succ"
)]
pub struct JsVariableDeclarationOp;

#[pliron_op(
    name = "js.function_declaration",
    interfaces = [
        NRegionsInterface<6>,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    attributes = (
        function_type: StringAttr,
        generator: BoolAttr,
        is_async: BoolAttr,
        function_is_declare: BoolAttr,
        pure: BoolAttr,
        pife: BoolAttr,
    ),
    format,
    verifier = "succ"
)]
pub struct JsFunctionDeclarationOp;

#[pliron_op(
    name = "js.class_declaration",
    interfaces = [
        NRegionsInterface<6>,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    attributes = (
        class_type: StringAttr,
        is_abstract: BoolAttr,
        declare: BoolAttr,
    ),
    format,
    verifier = "succ"
)]
pub struct JsClassDeclarationOp;

#[pliron_op(
    name = "js.ts_import_equals_declaration",
    interfaces = [
        NRegionsInterface<2>,
        SingleBlockRegionInterface,
        NoTerminatorInterface,
    ],
    attributes = (
        import_kind: StringAttr,
    ),
    format,
    verifier = "succ"
)]
pub struct JsTSTypeAliasDeclarationOp;

#[pliron_op(name = "js.ts_interface_declaration", format, verifier = "succ")]
pub struct JsTSInterfaceDeclarationOp;

#[pliron_op(name = "js.ts_enum_declaration", format, verifier = "succ")]
pub struct JsTSEnumDeclarationOp;

#[pliron_op(name = "js.ts_external_module_declaration", format, verifier = "succ")]
pub struct JsTSExternalModuleDeclarationOp;

#[pliron_op(name = "js.ts_namespace_declaration", format, verifier = "succ")]
pub struct JsTSNamespaceDeclarationOp;

#[pliron_op(name = "js.ts_global_declaration", format, verifier = "succ")]
pub struct JsTSGlobalDeclarationOp;

#[pliron_op(name = "js.ts_import_equals_declaration", format, verifier = "succ")]
pub struct JsTSImportEqualsDeclarationOp;

#[pliron_op(name = "js.import_declaration", format, verifier = "succ")]
pub struct JsImportDeclarationOp;

#[pliron_op(name = "js.export_all_declaration", format, verifier = "succ")]
pub struct JsExportAllDeclarationOp;

#[pliron_op(name = "js.export_default_declaration", format, verifier = "succ")]
pub struct JsExportDefaultDeclarationOp;

#[pliron_op(name = "js.export_declaration", format, verifier = "succ")]
pub struct JsExportDeclarationOp;

#[pliron_op(name = "js.export_named_declaration", format, verifier = "succ")]
pub struct JsExportNamedDeclarationOp;

#[pliron_op(name = "js.export_from_declaration", format, verifier = "succ")]
pub struct JsExportFromDeclarationOp;

#[pliron_op(name = "js.ts_export_assignment", format, verifier = "succ")]
pub struct JsTSExportAssignmentOp;

#[pliron_op(name = "js.ts_namespace_export_declaration", format, verifier = "succ")]
pub struct JsTSNamespaceExportDeclarationOp;
