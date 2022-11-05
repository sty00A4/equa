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
    let mut context = Context::new();
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
