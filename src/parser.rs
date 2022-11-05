use crate::set::*;
use crate::position::*;
use crate::error::*;
use crate::lexer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Int{ v: i64, pos: Position }, Float{ v: f64, pos: Position }, Word{ v: String, pos: Position },
    Vector{ v: Vec<Node>, pos: Position }, Set{ v: Set<Node>, pos: Position },
    Binary{ op: Token, left: Box<Node>, right: Box<Node>, pos: Position }, Unary{ op: Token, node: Box<Node>, pos: Position },
    Call{ v: Box<Node>, args: Vec<Node>, pos: Position }, Tuple{ nodes: Vec<Node>, pos: Position },
    Percent{ node: Box<Node>, pos: Position }, Abs{ node: Box<Node>, pos: Position },
    Assign { m: bool, id: Box<Node>, expr: Box<Node>, pos: Position }
}
impl Node {
    pub fn pos(&self) -> Position {
        match self {
            Self::Int { v: _, pos } => pos.clone(),
            Self::Float { v: _, pos } => pos.clone(),
            Self::Word { v: _, pos } => pos.clone(),
            Self::Vector { v: _, pos } => pos.clone(),
            Self::Set { v: _, pos } => pos.clone(),
            Self::Binary { op: _, left: _, right: _, pos } => pos.clone(),
            Self::Unary { op: _, node: _, pos } => pos.clone(),
            Self::Call { v: _, args: _, pos } => pos.clone(),
            Self::Tuple { nodes: _, pos } => pos.clone(),
            Self::Percent { node: _, pos } => pos.clone(),
            Self::Abs { node: _, pos } => pos.clone(),
            Self::Assign { m: _, id: _, expr: _, pos } => pos.clone(),
        }
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int { v, pos: _ } => write!(f, "{v}"),
            Self::Float { v, pos: _ } => write!(f, "{v}"),
            Self::Word { v, pos: _ } => write!(f, "{v}"),
            Self::Vector { v, pos: _ } => write!(f, "[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Set { v, pos: _ } => write!(f, "{{{}}}", v.values.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Binary { op, left, right, pos: _ } => write!(f, "({left} {op} {right})"),
            Self::Unary { op, node, pos: _ } => write!(f, "({op} {node})"),
            Self::Call { v, args, pos: _ } => write!(f, "{v}({})", args.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            Self::Tuple { nodes, pos: _ } => write!(f, "({})", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            Self::Percent { node, pos: _ } => write!(f, "{node}%"),
            Self::Abs { node, pos: _ } => write!(f, "|{node}|"),
            Self::Assign { m, id, expr, pos } => if *m { write!(f, "{id} := {expr}") } else { write!(f, "{id} :: {expr}") }
        }
    }
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub poses: Vec<Position>,
    pub idx: usize,
    pub path: String
}
impl Parser {
    pub fn new(tokens: Vec<Token>, poses: Vec<Position>, path: &str) -> Self {
        Self { tokens, poses, idx: 0, path: path.to_string() }
    }
    pub fn token(&self) -> Token {
        match self.tokens.get(self.idx) {
            Some(token) => token.clone(),
            None => Token::None
        }
    }
    pub fn pos(&self) -> Position {
        match self.poses.get(self.idx) {
            Some(pos) => pos.clone(),
            None => self.poses.last().unwrap().clone()
        }
    }
    pub fn expected(&self, token: Token) -> Result<(), Error> {
        if self.token() != token { return Err(Error::ExpectedToken(token)) }
        Ok(())
    }
    pub fn expected_unknown(&self, token: Token) -> Result<(), Error> {
        if self.token() != token { return Err(Error::UnexpectedToken(self.token())) }
        Ok(())
    }
    pub fn advance(&mut self) {
        self.idx += 1;
    }
    pub fn call(&mut self, f: &str) -> Result<Node, Error> {
        match f {
            "expr" => self.expr(),
            "merge" => self.merge(),
            "arith" => self.arith(),
            "term" => self.term(),
            "pow" => self.pow(),
            "factor" => self.factor(),
            "percent" => self.percent(),
            "amount" => self.amount(),
            "fcall" => self.fcall(),
            "field" => self.field(),
            "atom" => self.atom(),
            _ => Err(Error::Todo(f.to_string()))
        }
    }
    pub fn binary(&mut self, ops: Vec<Token>, f: &str) -> Result<Node, Error> {
        let mut left = self.call(f)?;
        let start = left.pos();
        while ops.contains(&self.token()) {
            let op = self.token();
            self.advance();
            let right = self.call(f)?;
            left = Node::Binary {
                op,
                left: Box::new(left.clone()),
                right: Box::new(right.clone()),
                pos: Position(start.0.clone(), start.1.start..right.pos().1.end)
            }
        }
        Ok(left)
    }
    pub fn parse(&mut self) -> Result<Node, Error> {
        let node = self.expr()?;
        self.expected_unknown(Token::None)?;
        Ok(node)
    }
    pub fn expr(&mut self) -> Result<Node, Error> {
        let id = self.comp()?;
        if self.token() == Token::Assign || self.token() == Token::Def {
            let tok = self.token();
            if let Node::Word { v, pos } = &id {
                self.advance();
                let expr = self.expr()?;
                return Ok(Node::Assign{
                    m: tok == Token::Assign,
                    id: Box::new(id.clone()),
                    expr: Box::new(expr.clone()),
                    pos: Position(self.path.clone(), pos.1.start..expr.pos().1.end)
                })
            } else {
                return Err(Error::UnexpectedToken(self.token()))
            }
        }
        Ok(id)
    }
    pub fn comp(&mut self) -> Result<Node, Error> {
        self.binary(vec![Token::Equal, Token::NotEqual, Token::Less, Token::Greater, Token::LessEqual, Token::GreaterEqual],
            "merge")
    }
    pub fn merge(&mut self) -> Result<Node, Error> {
        self.binary(vec![Token::Concat, Token::Remove], "arith")
    }
    pub fn arith(&mut self) -> Result<Node, Error> {
        self.binary(vec![Token::Plus, Token::Minus, Token::PlusMinus], "term")
    }
    pub fn term(&mut self) -> Result<Node, Error> {
        self.binary(vec![Token::Star, Token::Slash], "pow")
    }
    pub fn pow(&mut self) -> Result<Node, Error> {
        self.binary(vec![Token::Exponent], "factor")
    }
    pub fn factor(&mut self) -> Result<Node, Error> {
        if self.token() == Token::Minus {
            let start = self.pos();
            self.advance();
            let node = self.factor()?;
            return Ok(Node::Unary {
                op: Token::Minus,
                node: Box::new(node.clone()),
                pos: Position(self.path.clone(),
                start.1.start..node.pos().1.end)
            })
        }
        self.percent()
    }
    pub fn percent(&mut self) -> Result<Node, Error> {
        let mut node = self.amount()?;
        while self.token() == Token::Percent {
            let stop = self.pos();
            self.advance();
            node = Node::Percent { node: Box::new(node.clone()), pos: Position(self.path.clone(), node.pos().1.start..stop.1.end) }
        }
        Ok(node)
    }
    pub fn amount(&mut self) -> Result<Node, Error> {
        if self.token() == Token::Amount {
            let start = self.pos();
            self.advance();
            let node = self.fcall()?;
            return Ok(Node::Unary {
                op: Token::Amount,
                node: Box::new(node.clone()),
                pos: Position(self.path.clone(),
                start.1.start..node.pos().1.end)
            })
        }
        self.fcall()
    }
    pub fn fcall(&mut self) -> Result<Node, Error> {
        let node = self.field()?;
        if self.token() == Token::EvalIn {
            self.advance();
            let mut nodes: Vec<Node> = vec![];
            let pos = self.pos().1;
            let (start, mut stop) = (pos.start, pos.end);
            while self.token() != Token::EvalOut && self.token() != Token::None {
                let node_ = self.expr()?;
                stop = node_.pos().1.end;
                nodes.push(node_);
                if self.token() != Token::EvalOut {
                    self.expected(Token::Sep)?;
                    self.advance();
                }
            }
            self.expected(Token::EvalOut)?;
            self.advance();
            return Ok(Node::Call { v: Box::new(node), args: nodes, pos: Position(self.path.clone(), start..stop) })
        }
        Ok(node)
    }
    pub fn field(&mut self) -> Result<Node, Error> {
        self.binary(vec![Token::Field], "atom")
    }
    pub fn atom(&mut self) -> Result<Node, Error> {
        match self.token() {
            Token::Int(v) => {
                self.advance();
                Ok(Node::Int { v, pos: self.pos() })
            }
            Token::Float(v) => {
                self.advance();
                Ok(Node::Float { v, pos: self.pos() })
            }
            Token::Word(v) => {
                self.advance();
                Ok(Node::Word { v, pos: self.pos() })
            }
            Token::EvalIn => {
                let start = self.pos();
                self.advance();
                let node = self.expr()?;
                if self.token() == Token::Sep {
                    let mut nodes: Vec<Node> = vec![node];
                    while self.token() == Token::Sep {
                        self.advance();
                        let node = self.expr()?;
                        nodes.push(node);
                    }
                    self.expected(Token::EvalOut)?;
                    let stop = self.pos();
                    self.advance();
                    return Ok(Node::Tuple { nodes, pos: Position(self.path.clone(), start.1.start..stop.1.end) })
                }
                self.expected(Token::EvalOut)?;
                self.advance();
                Ok(node)
            }
            Token::VectorIn => {
                self.advance();
                let mut nodes: Vec<Node> = vec![];
                let pos = self.pos().1;
                let (start, mut stop) = (pos.start, pos.end);
                while self.token() != Token::VectorOut && self.token() != Token::None {
                    let node = self.expr()?;
                    stop = node.pos().1.end;
                    nodes.push(node);
                }
                self.expected(Token::VectorOut)?;
                self.advance();
                Ok(Node::Vector { v: nodes, pos: Position(self.path.clone(), start..stop) })
            }
            Token::SetIn => {
                self.advance();
                let mut nodes: Set<Node> = Set::new();
                let pos = self.pos().1;
                let (start, mut stop) = (pos.start, pos.end);
                while self.token() != Token::SetOut && self.token() != Token::None {
                    let node = self.expr()?;
                    stop = node.pos().1.end;
                    nodes.add(node);
                }
                self.expected(Token::SetOut)?;
                self.advance();
                Ok(Node::Set { v: nodes, pos: Position(self.path.clone(), start..stop) })
            }
            Token::Abs => {
                let start = self.pos();
                self.advance();
                let node = self.expr()?;
                self.expected(Token::Abs)?;
                let stop = self.pos();
                self.advance();
                Ok(Node::Abs{ node: Box::new(node), pos: Position(self.path.clone(), start.1.start..stop.1.end) })
            }
            _ => Err(Error::UnexpectedToken(self.token()))
        }
    }
}

pub fn parse(tokens: Vec<Token>, poses: Vec<Position>, path: &str) -> Result<Node, Error> {
    Parser::new(tokens, poses, path).parse()
}