use neon::prelude::*;
use lynxlang::env::Env;
use lynxlang::evaluator::Evaluator;
use lynxlang::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;

fn run(mut cx: FunctionContext) -> JsResult<JsString> {
    // https://neon-bindings.com/docs/functions
    let handles = cx.argument::<JsString>(0)?;
    let input = handles.value(&mut cx);
    let program = Parser::get(&input.as_str()).parse_program();
    let mut evaluator = Evaluator::new(Rc::new(RefCell::new(Env::new())));
    
    evaluator.builtin();

    match evaluator.eval_program(program) {
        Some(value) => Ok(cx.string(format!("{:?}", value))),
        None => Ok(cx.string("Null")),
    }    
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("run", run)?;
    Ok(())
}
