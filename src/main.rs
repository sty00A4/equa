#![allow(dead_code)]
#![allow(unused)]

extern crate logos;
mod set;
mod position;
mod value; use value::*;
mod error; use error::*;
mod lexer;
mod parser;
mod interpreter; use interpreter::*;
use std::{io, io::Write};

pub fn run(text: &str, context: &mut Context, path: &str) -> Result<Option<Value>, Error> {
    let (tokens, poses) = lexer::lex(text, path)?;
    if tokens.len() == 0 { return Ok(None) }
    let node = parser::parse(tokens, poses, path)?;
    let value = interpreter::get(&node, context, path)?;
    Ok(Some(value))
}

fn main() {
    let mut context = Context::new(); std_context(&mut context);
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        let res = run(input.as_str(), &mut context, "stdin");
        if let Err(e) = res {
            println!("{e}");
        } else {
            if let Some(v) = res.unwrap() { println!("{v}"); }
        }
    }
}

pub fn _sqrt(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).sqrt())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.sqrt())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _floor(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(value.clone())
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.floor())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _ceil(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(value.clone())
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.ceil())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _round(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(value.clone())
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.round())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn std_context(context: &mut Context) {
    context.global_const(&"sqrt".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _sqrt));
    context.global_const(&"floor".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _floor));
    context.global_const(&"ceil".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _ceil));
    context.global_const(&"round".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _round));
}