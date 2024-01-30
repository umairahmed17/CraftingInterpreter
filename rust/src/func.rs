use crate::{interpreter::Interpreter, expr::Value};


trait Callable{
    fn call(interpreter: &Interpreter, args: Vec<Value>) -> Value;
}
