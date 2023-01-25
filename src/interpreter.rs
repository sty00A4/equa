use crate::set::*;
use crate::position::*;
use crate::error::*;
use crate::lexer::*;
use crate::parser::*;
use crate::value::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, Value>,
    consts: HashMap<String, Value>
}
impl Scope {
    pub fn new() -> Self { Self { vars: HashMap::new(), consts: HashMap::new() } }
    pub fn get(&self, id: &String) -> Option<&Value> {
        self.vars.get(id).or_else(|| self.consts.get(id))
    }
    pub fn is_const(&self, id: &String) -> bool {
        self.consts.get(id).is_some()
    }
    pub fn set(&mut self, id: &String, value: &Value) -> Option<Value> {
        self.vars.insert(id.clone(), value.clone())
    }
    pub fn set_const(&mut self, id: &String, value: &Value) -> Option<Value> {
        self.consts.insert(id.clone(), value.clone())
    }
}

#[derive(Debug)]
pub struct Context {
    scopes: Vec<Scope>,
    global: Scope,
}
impl Context {
    pub fn new() -> Self { Self { scopes: vec![Scope::new()], global: Scope::new() } }
    pub fn get(&self, id: &String) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            let v = scope.get(id);
            if v.is_some() { return v }
        }
        self.global.get(id)
    }
    pub fn set(&mut self, id: &String, value: &Value) -> Result<(), ()> {
        if self.global.get(id).is_some() { return Err(()) }
        for scope in self.scopes.iter_mut() {
            if scope.get(id).is_some() {
                if scope.is_const(id) { return Err(()) }
                let v = scope.set(id, value);
                return Ok(())
            }
        }
        self.scopes.last_mut().unwrap().set(id, value);
        Ok(())
    }
    pub fn set_const(&mut self, id: &String, value: &Value) -> Result<(), ()> {
        if self.global.get(id).is_some() { return Err(()) }
        for scope in self.scopes.iter() {
            if scope.get(id).is_some() { return Err(()) }
        }
        self.scopes.last_mut().unwrap().set_const(id, value);
        Ok(())
    }
    pub fn global(&mut self, id: &String, value: &Value) -> Result<(), ()> {
        if self.global.get(id).is_some() { return Err(()) }
        for scope in self.scopes.iter() {
            if scope.get(id).is_some() { return Err(()) }
        }
        self.global.set(id, value);
        Ok(())
    }
    pub fn global_const(&mut self, id: &String, value: &Value) -> Result<(), ()> {
        if self.global.get(id).is_some() { return Err(()) }
        for scope in self.scopes.iter() {
            if scope.get(id).is_some() { return Err(()) }
        }
        self.global.set_const(id, value);
        Ok(())
    }
}

pub fn get(node: &Node, context: &mut Context, path: &str) -> Result<Value, Error> {
    match node {
        Node::Int { v, pos } => Ok(Value::Number(Number::Int(*v))),
        Node::Float { v, pos } => Ok(Value::Number(Number::Float(*v))),
        Node::Percent { node, pos } => {
            let value = get(node, context, path)?;
            if let Value::Number(v) = value {
                return Ok(Value::Number(v / Number::Int(100)))
            }
            Err(Error::ExpectedType(Type::Number, value.typ()))
        }
        Node::Abs { node, pos } => {
            let value = get(node, context, path)?;
            if let Value::Number(v) = value {
                return Ok(Value::Number(if v < Number::Int(0) { -v } else { v }))
            }
            Err(Error::ExpectedTypes(vec![Type::Number, Type::Set], value.typ()))
        }
        Node::Word { v, pos } => {
            let value = context.get(v);
            if value.is_none() { return Err(Error::NotDefined(v.clone())) }
            Ok(value.unwrap().clone())
        }
        Node::Vector { v, pos } => {
            let mut values: Vec<Value> = vec![];
            for n in v.iter() {
                let value = get(n, context, path)?;
                values.push(value);
            }
            Ok(Value::Vector(values))
        }
        Node::Set { v, pos } => {
            let mut values: Set<Value> = Set::new();
            for n in v.values.iter() {
                let value = get(n, context, path)?;
                values.add(value);
            }
            Ok(Value::Set(values))
        }
        Node::Binary { op, left, right, pos } => {
            let v1 = get(left.as_ref(), context, path)?;
            let v2 = get(right.as_ref(), context, path)?;
            let value = v1.binop(op, &v2);
            if value.is_none() { return Err(Error::IllegalBinaryOperation(op.clone(), v1, v2)) }
            Ok(value.unwrap())
        }
        Node::Unary { op, node, pos } => {
            let v = get(node.as_ref(), context, path)?;
            let value = v.unop(op);
            if value.is_none() { return Err(Error::IllegalUnaryOperation(op.clone(), v)) }
            Ok(value.unwrap())
        }
        Node::Call { v, args, pos } => {
            let func = get(v, context, path)?;
            if let Value::Function(params, body) = func {
                let global = context.global.clone();
                let mut fcontext = Context::new(); fcontext.global = global;
                for i in 0..params.len() {
                    if let Some(n) = args.get(i) {
                        let value = get(n, context, path)?;
                        fcontext.set(&params[i], &value);
                    } else {
                        return Err(Error::TooFewArgs(params.len(), args.len()))
                    }
                }
                return get(&body, &mut fcontext, path)
            }
            if let Value::ForeignFunction(params, function) = func {
                let global = context.global.clone();
                let mut fcontext = Context::new(); fcontext.global = global;
                for i in 0..params.len() {
                    if let Some(n) = args.get(i) {
                        let value = get(n, context, path)?;
                        fcontext.set(&params[i], &value);
                    } else {
                        return Err(Error::TooFewArgs(params.len(), args.len()))
                    }
                }
                return function(&mut fcontext)
            }
            Err(Error::ExpectedType(Type::Function, func.typ()))
        }
        Node::Tuple { nodes, pos } => {
            let mut values: Vec<Value> = vec![];
            for n in nodes.iter() {
                let value = get(n, context, path)?;
                values.push(value);
            }
            Ok(Value::Tuple(values))
        }
        Node::Function { id, body, pos } => {
            if let Node::Word { v, pos } = id.as_ref() {
                return Ok(Value::Function(vec![v.clone()], body.as_ref().clone()))
            } else if let Node::Tuple { nodes, pos } = id.as_ref() {
                let mut params: Vec<String> = vec![];
                for n in nodes.iter() {
                    if let Node::Word { v, pos } = n {
                        params.push(v.clone());
                    } else {
                        return Err(Error::UnexpectedNode(n.clone()))
                    }
                }
                return Ok(Value::Function(params, body.as_ref().clone()))
            }
            Err(Error::UnexpectedNode(id.as_ref().clone()))
        }
        Node::Assign { m, id, expr, pos } => {
            let value = get(expr.as_ref(), context, path)?;
            if let Node::Word { v, pos } = id.as_ref() {
                if *m {
                    let res = context.set(v, &value);
                    if res.is_err() { return Err(Error::Immutable(v.clone())) }
                } else {
                    let res = context.set_const(v, &value);
                    if res.is_err() { return Err(Error::AlreadyDefined(v.clone())) }
                }
                return Ok(value)
            }
            Err(Error::UnexpectedNode(id.as_ref().clone()))
        }
    }
}