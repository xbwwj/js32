//! This example manually construct the following program and print.
//!
//! ```js
//! #!/usr/bin/env node
//! "use strict";
//! debugger;
//! ;
//! ```

use pliron::{
    basic_block::BasicBlock,
    builtin::attributes::StringAttr,
    common_traits::Verify,
    context::{Context, Ptr},
    identifier::Identifier,
    op::Op,
    operation::Operation,
    printable::Printable,
};

use js_dialect::ops::{JsDebuggerStatementOp, JsDirectiveOp, JsEmptyStatementOp, JsProgramOp};

fn create_op<O: Op>(ctx: &mut Context, num_regions: usize) -> Ptr<Operation> {
    Operation::new(
        ctx,
        O::get_concrete_op_info(),
        vec![],
        vec![],
        vec![],
        num_regions,
    )
}

fn set_string_attr(ctx: &Context, op: Ptr<Operation>, name: &'static str, value: &str) {
    op.deref_mut(ctx)
        .attributes
        .set(Identifier::new(name), StringAttr::new(value.to_string()));
}

fn main() {
    let mut ctx = Context::new();

    // 1. js.program
    let program = create_op::<JsProgramOp>(&mut ctx, 2);

    set_string_attr(&ctx, program, "hashbang", "#!/usr/bin/env node");

    // 2. Regions
    let directives_region = program.deref(&ctx).get_region(0);
    let body_region = program.deref(&ctx).get_region(1);

    // 3. BasicBlocks
    let directives_block = BasicBlock::new(&mut ctx, None, vec![]);
    let body_block = BasicBlock::new(&mut ctx, None, vec![]);

    directives_block.insert_at_back(directives_region, &ctx);
    body_block.insert_at_back(body_region, &ctx);

    // 4. Directive
    let directive = create_op::<JsDirectiveOp>(&mut ctx, 0);

    set_string_attr(&ctx, directive, "expression", "use strict");

    directive.insert_at_back(directives_block, &ctx);

    // 5. Body statements
    let debugger = create_op::<JsDebuggerStatementOp>(&mut ctx, 0);
    debugger.insert_at_back(body_block, &ctx);

    let empty = create_op::<JsEmptyStatementOp>(&mut ctx, 0);
    empty.insert_at_back(body_block, &ctx);

    // 6. Verify
    program.verify(&ctx).expect("Invalid JS program");

    // 7. Print
    println!("{}", program.disp(&ctx));
}
