#[allow(dead_code)]

fn is_valid_keyword(s: &str) -> bool {
    matches!(s, "if" | "while" | "else" | "elif" | "endif")
}
fn is_valid_symbol(s: &str) -> bool {
    matches!(s, "=" | "(" | ")")
}
fn is_valid_operator(s: &str) -> bool {
    matches!(s, "+" | "-" | "*" | "/")
}
fn is_valid_token(s: &str) -> bool {
    s.is_empty()
        || s.chars().all(char::is_alphanumeric)
        || is_valid_keyword(s)
        || is_valid_symbol(s)
        || is_valid_operator(s)
}

fn find_delimiter<F>(s: &str, op: F) -> (&str, Option<&str>)
where
    F: Fn(char) -> bool,
{
    let delim = s.find(op);
    match delim {
        Some(delim) => {
            let (first_token, remainder) = s.split_at(delim);
            (first_token, Some(remainder))
        }
        None => (s, None),
    }
}


fn get_first_token(str: &str) -> (&str, Option<&str>) {
    let s = str.trim();
    let first = s.chars().next();
    match first {
        Some(first) => {
            if !is_valid_token(&first.to_string()) {
                panic!("invalid token at{:?}", first);
            } 
            else if first.is_numeric() {
                find_delimiter(s, |c| !c.is_numeric())
            } else if first.is_alphabetic() {
                find_delimiter(s, |c| !c.is_alphanumeric())
            } else if is_valid_symbol(&first.to_string()) || is_valid_operator(&first.to_string()) {
                (&s[0..1], Some(&s[1..]))
            }else{
                panic!("wtf idk what to do here ngl")
            } 
        }
        None => (s, None)
    }
}
fn tokenize(s: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let (mut first, mut remainder) = get_first_token(&s);
    loop{
        tokens.push(first);
        if remainder == None {
            break;
        }else{
            (first,remainder) = get_first_token(remainder.unwrap());
        }
    } 
    tokens
}


//////AST stuff//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Token<'a> {
    Int(i32),
    Keyword(&'a str),
    Operator(char),
    Symbol(char),
    Identifier(&'a str),
}

fn to_token(s: &str) -> Token {
    if s.chars().next().unwrap().is_numeric() {
        Token::Int(s.parse().unwrap())
    } else if is_valid_keyword(s) {
        Token::Keyword(s)
    } else if is_valid_symbol(&s) {
        Token::Operator(s.chars().next().unwrap())
    } else if is_valid_operator(&s) {
        Token::Operator(s.chars().next().unwrap())
    } else if s.chars().next().unwrap().is_alphabetic() {
        Token::Identifier(s)
    } else {
        panic!("wtf is {}", s)
    }
}

impl <'a>Token<'a> {
    fn print_token(&self) {
        match self {
            Token::Int(val) => {
                println!("type: Integer ");
                println!("value : {val}");
            }
            Token::Keyword(val) => {
                println!("type: keyword ");
                println!("value : {val}");
            }
            Token::Operator(val) => {
                println!("type: operator ");
                println!("value : {val}");
            }
            Token::Symbol(val) => {
                println!("type: operator ");
                println!("value : {val}");
            }
            Token::Identifier(val) => {
                println!("type: identifier ");
                println!("value : {val}");
            }
        }
    }
}

enum Tree<'a> {
    EmptyTree,
    Node {
        Value: Token<'a>,
        left: Box<Tree<'a>>,
        right: Box<Tree<'a>>,
    },
}



fn main() {
    let result = tokenize("ass + 7");
    //result.iter().for_each(|x| println!("{}", x));
    result
    .iter()
    .map(|x| to_token(x))
    .for_each(|x| x.print_token());
}
