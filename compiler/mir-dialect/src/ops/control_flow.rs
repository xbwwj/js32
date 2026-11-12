use pliron::{
    builtin::op_interfaces::{IsTerminatorInterface, NOpdsInterface, NResultsInterface},
    derive::pliron_op,
};

#[pliron_op(
    name = "mir.return",
    format,
    interfaces = [NResultsInterface<0>, IsTerminatorInterface],
    verifier = "succ",
)]
pub struct MirReturnOp;

#[pliron_op(
    name = "mir.goto",
    format,
    interfaces = [NResultsInterface<0>, IsTerminatorInterface],
    verifier = "succ",
)]
pub struct MirGotoOp;

#[pliron_op(
    name = "mir.cond_br",
    format,
    interfaces = [NResultsInterface<0>, IsTerminatorInterface],
    verifier = "succ",
)]
pub struct MirCondBranchOp;

#[pliron_op(
    name = "mir.assert",
    format,
    interfaces = [NResultsInterface<0>],
    verifier = "succ",
)]
pub struct MirAssertOp;

#[pliron_op(
    name = "mir.unreachable",
    format = "",
    interfaces = [NOpdsInterface<0>, NResultsInterface<0>, IsTerminatorInterface],
    verifier = "succ",
)]
pub struct MirUnreachableOp;
