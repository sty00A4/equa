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
pub fn _abs(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Int(v.abs())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.abs())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _sin(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).sin())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.sin())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _cos(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).cos())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.cos())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _tan(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).tan())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.tan())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _asin(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).asin())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.asin())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _acos(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).acos())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.acos())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _atan(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).atan())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.atan())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _atan2(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    let other = context.get(&"y".to_string()).unwrap();
    if let (Value::Number(num), Value::Number(num2)) = (value, other) {
        match (num, num2) {
            (Number::Int(v1), Number::Int(v2)) => return Ok(Value::Number(Number::Float((*v1 as f64).atan2((*v1 as f64))))),
            (Number::Float(v1), Number::Float(v2)) => return Ok(Value::Number(Number::Float(v1.atan2(*v2)))),
            (Number::Int(v1), Number::Float(v2)) => return Ok(Value::Number(Number::Float((*v1 as f64).atan2(*v2)))),
            (Number::Float(v1), Number::Int(v2)) => return Ok(Value::Number(Number::Float(v1.atan2((*v2 as f64))))),
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _sinh(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).sinh())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.sinh())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _cosh(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).cosh())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.cosh())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _tanh(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).tanh())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.tanh())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _asinh(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).asinh())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.asinh())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _acosh(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).acosh())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.acosh())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn _atanh(context: &mut Context) -> Result<Value, Error> {
    let value = context.get(&"x".to_string()).unwrap();
    if let Value::Number(num) = value {
        if let Number::Int(v) = num {
            return Ok(Value::Number(Number::Float((*v as f64).atanh())))
        }
        if let Number::Float(v) = num {
            return Ok(Value::Number(Number::Float(v.atanh())))
        }
    }
    Err(Error::ExpectedType(Type::Number))
}
pub fn std_context(context: &mut Context) {
    context.global_const(&"sqrt".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _sqrt));
    context.global_const(&"floor".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _floor));
    context.global_const(&"ceil".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _ceil));
    context.global_const(&"round".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _round));
    context.global_const(&"abs".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _abs));
    context.global_const(&"sin".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _sin));
    context.global_const(&"cos".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _cos));
    context.global_const(&"tan".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _tan));
    context.global_const(&"asin".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _asin));
    context.global_const(&"acos".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _acos));
    context.global_const(&"atan".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _atan));
    context.global_const(&"atan2".to_string(), &Value::ForeignFunction(vec!["x".to_string(), "y".to_string()], _atan2));
    context.global_const(&"sin".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _sinh));
    context.global_const(&"cosh".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _cosh));
    context.global_const(&"tanh".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _tanh));
    context.global_const(&"asinh".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _asinh));
    context.global_const(&"acosh".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _acosh));
    context.global_const(&"atanh".to_string(), &Value::ForeignFunction(vec!["x".to_string()], _atanh));
    context.global_const(&"PI".to_string(), &Value::Number(Number::Float(std::f64::consts::PI)));
    context.global_const(&"TAU".to_string(), &Value::Number(Number::Float(std::f64::consts::TAU)));
    context.global_const(&"E".to_string(), &Value::Number(Number::Float(std::f64::consts::E)));
}