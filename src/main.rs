#[allow(dead_code)]

fn is_valid_keyword(s:&str)->bool{
    ["if","while","else","elif","endif","endwhile"].contains(&s)
}
fn is_valid_symbol(c:char)->bool{
    ['+','-','*','/','='].contains(&c)
}
fn is_valid_token(s: &str)->bool{
   return s.is_empty() || s.chars().all(char::is_alphanumeric) || is_valid_keyword(s);
}

fn find_delimiter <F> (s:&str, op:F)-> (&str,&str) where F: Fn(char)->bool, {
    let delim = s.chars().position(op);
    match delim {
        Some(delim)=>{
            let (first_token,remainder) = s.split_at(delim);
            (first_token,remainder)
        }
        None => {(s," ")} 
    } 
} 

fn get_first_token(str:&str)->(&str,&str){
    let s = str.trim();
    let first = s.chars().next();
    match first{
    Some(first)=>{
            if first.is_numeric(){
                find_delimiter(s, |c|!c.is_numeric())
            }
            else if first.is_alphabetic(){
                find_delimiter(s, |c| ! c.is_alphanumeric())  
            }
            else if is_valid_symbol(first){
                s.split_at(1) 
            }
            else{
                (" ", " ")
            }
        }
    None => {return (s, " ");}
    
    }   
}
fn tokenize(s:&str, tokens: &mut Vec<String>)->Vec<String>{
    let (first,remainder) = get_first_token(&s);
    if remainder.eq(" "){
        tokens.push(first.to_string());
        return tokens.iter().map(|s|s.to_string()).collect();
    }else{
        tokens.push(first.to_string());
        tokenize(&remainder, tokens)
    }
}

fn tokenize_real (s:&str)->Vec<String>{
    tokenize(s, &mut Vec::new())
}
fn main() {
    //let mut tokens = Vec::new();
    let result = tokenize_real("112peepee+=-yourmom69-");
    result.iter().for_each(|x|println!("{}",x))
}
