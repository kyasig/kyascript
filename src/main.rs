#[allow(dead_code)]

fn is_valid_keyword(s: &str) -> bool {
    matches!(s, "if" | "while" | "else" | "elif" | "endif")
}
fn is_valid_symbol(s: &str) -> bool {
    matches!(s, "+" | "-" | "*" | "/" | "=" | "(" | ")")
}
fn is_valid_token(s: &str) -> bool {
    s.is_empty()
        || s.chars().all(char::is_alphanumeric)
        || is_valid_keyword(s)
        || is_valid_symbol(s)
}

fn find_delimiter<F>(s: &str, op: F) -> (&str, &str)
where
    F: Fn(char) -> bool,
{
    let delim = s.chars().position(op);
    match delim {
        Some(delim) => {
            let (first_token, remainder) = s.split_at(delim);
            (first_token, remainder)
        }
        None => (s, " "),
    }
}

fn get_first_token(str: &str) -> (&str, &str) {
    let s = str.trim();
    let first = s.chars().next();
    match first {
        Some(first) => {
            if !is_valid_token(&first.to_string()) {
                panic!("{}", first);
            } else if first.is_numeric() {
                find_delimiter(s, |c| !c.is_numeric())
            } else if first.is_alphabetic() {
                find_delimiter(s, |c| !c.is_alphanumeric())
            } else if is_valid_symbol(&first.to_string()) {
                s.split_at(1)
            } else {
                (" ", " ")
            }
        }
        None => (s, " "),
    }
}
fn tokenize(s: &str, tokens: &mut Vec<String>) -> Vec<String> {
    let (first, remainder) = get_first_token(&s);
    tokens.push(first.to_string());
    if remainder.eq(" ") {
        return tokens.iter().map(|s| s.to_string()).collect();
    } else {
        tokenize(&remainder, tokens)
    }
}

fn tokenize_real(s: &str) -> Vec<String> {
    tokenize(s, &mut Vec::new())
}

//////AST stuff//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Token {
    Int(i32),
    Keyword(String),
    Operator(char),
    Symbol(char),
    Identifier(String),
}

fn to_token(s: String) -> Token {
    if s.chars().next().unwrap().is_numeric() {
        Token::Int(s.parse().unwrap())
    } else if is_valid_keyword(s.as_str()) {
        Token::Keyword(s)
    } else if is_valid_symbol(&s) {
        Token::Operator(s.chars().next().unwrap())
    } else if is_valid_symbol(s.as_str()) {
        Token::Symbol(s.chars().next().unwrap())
    } else {
        Token::Identifier(s)
    }
}

impl Token {
    fn print_token(&self) {
        match self {
            Token::Int(val) => {
                println!("type: Integer ");
                println!("value : {}", val);
            }
            Token::Keyword(val) => {
                println!("type: keyword ");
                println!("value : {}", val);
            }
            Token::Operator(val) => {
                println!("type: operator ");
                println!("value : {}", val);
            }
            Token::Symbol(val) => {
                println!("type: operator ");
                println!("value : {}", val);
            }
            Token::Identifier(val) => {
                println!("type: identifier ");
                println!("value : {}", val);
            }
        }
    }
}

enum Tree {
    EmptyTree,
    Node {
        Value: Token,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Tree {
    fn append(self, token: Token) -> Tree {
        match self {
            Tree::EmptyTree => match token {
                Token::Operator(_) | Token::Symbol(_) => panic!("invalid initial token"),
                Token::Int(_) | Token::Identifier(_) | Token::Keyword(_) => Tree::Node {
                    Value: token,
                    left: Box::new(Tree::EmptyTree),
                    right: Box::new(Tree::EmptyTree),
                },
            },
            Tree::Node {
                Value: Token::Int(_),
                ..
            }
            | Tree::Node {
                Value: Token::Identifier(_),
                ..
            } => {
                todo!()
            }
            Tree::Node {
                Value: Token::Operator(_),
                ..
            }
            | Tree::Node {
                Value: Token::Symbol(_),
                ..
            }
            | Tree::Node {
                Value: Token::Keyword(_),
                ..
            } => todo!(),
        }
    }
}
fn main() {
    let result = tokenize_real(" 5 + 7");
    result
        .iter()
        .map(|x| to_token(x.to_string()))
        .for_each(|x| x.print_token());
}
