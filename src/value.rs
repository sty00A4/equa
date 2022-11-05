use std::cmp::min;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::lexer::Token;
use crate::set::*;
use crate::error::*;

#[derive(Clone, Debug)]
pub enum Number { Int(i64), Float(f64) }
impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
        }
    }
}
impl Add for Number {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 + v2),
            (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 + v2),
            (Self::Int(v1), Self::Float(v2)) => Self::Float(v1 as f64 + v2),
            (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 + v2 as f64),
        }
    }
}
impl Sub for Number {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 - v2),
            (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 - v2),
            (Self::Int(v1), Self::Float(v2)) => Self::Float(v1 as f64 - v2),
            (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 - v2 as f64),
        }
    }
}
impl Mul for Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 * v2),
            (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 * v2),
            (Self::Int(v1), Self::Float(v2)) => Self::Float(v1 as f64 * v2),
            (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 * v2 as f64),
        }
    }
}
impl Div for Number {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => Self::Float(v1 as f64 / v2 as f64),
            (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 / v2),
            (Self::Int(v1), Self::Float(v2)) => Self::Float(v1 as f64 / v2),
            (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 / v2 as f64),
        }
    }
}
impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Int(v) => Self::Int(-v),
            Self::Float(v) => Self::Float(-v),
        }
    }
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => *v1 == *v2,
            (Self::Float(v1), Self::Float(v2)) => *v1 == *v2,
            (Self::Float(v1), Self::Int(v2)) => *v1 == *v2 as f64,
            (Self::Int(v1), Self::Float(v2)) => *v1 as f64 == *v2,
            _ => false,
        }
    }
}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(v1), Self::Int(v2)) => if *v1 < *v2 {
                Some(std::cmp::Ordering::Less)
            } else if *v1 > *v2 {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
            (Self::Float(v1), Self::Float(v2)) => if *v1 < *v2 {
                Some(std::cmp::Ordering::Less)
            } else if *v1 > *v2 {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
            (Self::Int(v1), Self::Float(v2)) => if (*v1 as f64) < *v2 {
                Some(std::cmp::Ordering::Less)
            } else if *v1 as f64 > *v2 {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
            (Self::Float(v1), Self::Int(v2)) => if *v1 < *v2 as f64 {
                Some(std::cmp::Ordering::Less)
            } else if *v1 > *v2 as f64 {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type { Number, Vector, Set, Tuple }
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number => write!(f, "number"),
            Self::Vector => write!(f, "vector"),
            Self::Set => write!(f, "set"),
            Self::Tuple => write!(f, "tuple"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value { Number(Number), Vector(Vec<Self>), Set(Set<Self>), Tuple(Vec<Self>) }
impl Value {
    pub fn typ(&self) -> Type {
        match self {
            Self::Number(_) => Type::Number,
            Self::Vector(_) => Type::Vector,
            Self::Set(_) => Type::Set,
            Self::Tuple(_) => Type::Tuple,
        }
    }
    pub fn unop(&self, op: &Token) -> Option<Self> {
        match self {
            Self::Number(v) => match op {
                Token::Minus => Some(Self::Number(-v.to_owned())),
                _ => None
            }
            Self::Vector(v) => match op {
                Token::Minus => {
                    let mut values: Vec<Self> = vec![];
                    for i in 0..v.len() {
                        values.push(v[i].unop(op)?);
                    }
                    Some(Self::Vector(values))
                }
                Token::Amount => {
                    Some(Self::Number(Number::Int(v.len() as i64)))
                }
                _ => None
            }
            Self::Set(v) => match op {
                Token::Minus => {
                    let mut values: Set<Self> = Set::new();
                    for i in 0..v.len() {
                        values.add(v.values[i].unop(op)?);
                    }
                    Some(Self::Set(values))
                }
                Token::Amount => {
                    Some(Self::Number(Number::Int(v.len() as i64)))
                }
                _ => None
            }
            _ => None
        }
    }
    pub fn binop(&self, op: &Token, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Number(v1), Self::Number(v2)) => match op {
                Token::Plus => Some(Self::Number(v1.to_owned() + v2.to_owned())),
                Token::Minus => Some(Self::Number(v1.to_owned() - v2.to_owned())),
                Token::PlusMinus => Some(Self::Tuple(vec![Self::Number(v1.to_owned() + v2.to_owned()), Self::Number(v1.to_owned() - v2.to_owned())])),
                Token::Star => Some(Self::Number(v1.to_owned() * v2.to_owned())),
                Token::Slash => Some(Self::Number(v1.to_owned() / v2.to_owned())),
                Token::Equal => Some(Self::Number(Number::Int((v1.to_owned() == v2.to_owned()) as i64))),
                Token::NotEqual => Some(Self::Number(Number::Int((v1.to_owned() != v2.to_owned()) as i64))),
                Token::Less => Some(Self::Number(Number::Int((v1.to_owned() < v2.to_owned()) as i64))),
                Token::Greater => Some(Self::Number(Number::Int((v1.to_owned() > v2.to_owned()) as i64))),
                Token::LessEqual => Some(Self::Number(Number::Int((v1.to_owned() <= v2.to_owned()) as i64))),
                Token::GreaterEqual => Some(Self::Number(Number::Int((v1.to_owned() >= v2.to_owned()) as i64))),
                _ => None
            }
            (Self::Vector(v1), Self::Vector(v2)) => match op {
                Token::Concat => {
                    let mut values: Vec<Self> = v1.clone();
                    for v in v2.iter() {
                        values.push(v.clone());
                    }
                    Some(Self::Vector(values))
                }
                _ => {
                    let mut values: Vec<Self> = vec![];
                    for i in 0..min(v1.len(), v2.len()) {
                        values.push(v1[i].binop(op, &v2[i])?);
                    }
                    Some(Self::Vector(values))
                }
            }
            (Self::Set(v1), Self::Set(v2)) => match op {
                Token::Concat => {
                    let mut values: Set<Self> = v1.clone();
                    for v in v2.values.iter() {
                        values.add(v.clone());
                    }
                    Some(Self::Set(values))
                }
                Token::Remove => {
                    let mut values: Set<Self> = v1.clone();
                    for v in v2.values.iter() {
                        values.remove(v);
                    }
                    Some(Self::Set(values))
                }
                _ => {
                    let mut values: Set<Self> = Set::new();
                    for i in 0..min(v1.len(), v2.len()) {
                        values.add(v1.values[i].binop(op, &v2.values[i])?);
                    }
                    Some(Self::Set(values))
                }
            }
            (Self::Tuple(v1), Self::Tuple(v2)) => match op {
                Token::Concat => {
                    let mut values: Vec<Self> = v1.clone();
                    for v in v2.iter() {
                        values.push(v.clone());
                    }
                    Some(Self::Tuple(values))
                }
                _ => {
                    let mut values: Vec<Self> = vec![];
                    for i in 0..min(v1.len(), v2.len()) {
                        values.push(v1[i].binop(op, &v2[i])?);
                    }
                    Some(Self::Tuple(values))
                }
            }
            (Self::Vector(v1), Self::Number(v2)) => match op {
                Token::Concat => {
                    let mut values: Vec<Self> = v1.clone();
                    values.push(other.clone());
                    Some(Self::Vector(values))
                }
                Token::Remove => {
                    let mut values: Vec<Self> = vec![];
                    for v in v1.iter() {
                        if v.to_owned() != Value::Number(v2.clone()) {
                            values.push(v.clone());
                        }
                    }
                    Some(Self::Vector(values))
                }
                _ => {
                    let mut values: Vec<Self> = vec![];
                    for i in 0..v1.len() {
                        values.push(v1[i].binop(op, other)?);
                    }
                    Some(Self::Vector(values))
                }
            }
            (Self::Set(v1), Self::Number(v2)) => match op {
                Token::Concat => {
                    let mut values: Set<Self> = v1.clone();
                    values.add(other.clone());
                    Some(Self::Set(values))
                }
                Token::Remove => {
                    let mut values: Set<Self> = v1.clone();
                    values.remove(other);
                    Some(Self::Set(values))
                }
                _ => {
                    let mut values: Set<Self> = Set::new();
                    for i in 0..v1.len() {
                        values.add(v1.values[i].binop(op, other)?);
                    }
                    Some(Self::Set(values))
                }
            }
            (Self::Tuple(v1), Self::Number(v2)) => match op {
                Token::Concat => {
                    let mut values: Vec<Self> = v1.clone();
                    values.push(other.clone());
                    Some(Self::Tuple(values))
                }
                Token::Remove => {
                    let mut values: Vec<Self> = v1.clone();
                    if let Number::Int(idx) = v2 {
                        values.remove(*idx as usize);
                        return Some(Self::Tuple(values))
                    }
                    None
                }
                _ => {
                    let mut values: Vec<Self> = vec![];
                    for i in 0..v1.len() {
                        values.push(v1[i].binop(op, other)?);
                    }
                    Some(Self::Tuple(values))
                }
            }
            _ => None
        }
    }
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(v) => write!(f, "{v}"),
            Self::Vector(v) => write!(f, "[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Set(v) => write!(f, "{{{}}}", v.values.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Tuple(v) => write!(f, "({})", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(v1), Self::Number(v2)) => v1 == v2,
            (Self::Vector(v1), Self::Vector(v2)) => v1 == v2,
            (Self::Set(v1), Self::Set(v2)) => v1 == v2,
            _ => false,
        }
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(v1), Self::Number(v2)) => if *v1 < *v2 {
                Some(std::cmp::Ordering::Less)
            } else if *v1 > *v2 {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
            _ => None
        }
    }
}