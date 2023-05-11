#[allow(dead_code)]

fn is_valid_keyword(s:&str)->bool{
    return ["if","while","else","elif","endif","endwhile"].contains(&s);
}
fn is_valid_symbol(c:char)->bool{
    return ['+','-','*','/','='].contains(&c);
}
fn is_valid_token(s: &str)->bool{
   return s.is_empty() || s.chars().all(char::is_alphanumeric) || is_valid_keyword(s);
}

fn find_delimiter <F> (s:&str, op:F)-> (&str,&str) where F: Fn(char)->bool, {
    let delim = s.chars().position(op);
    match delim {
        Some(delim)=>{
        let (first_token,remainder) = s.split_at(delim);
        return (first_token,remainder);
        }
        None => {return (s," ");} 
    } 
} 

fn get_first_token(s:&str)->(&str,&str){
    let first = s.chars().next();
    match first{
    Some(first)=>{
            if first.is_numeric(){
            return find_delimiter(s, |c|!c.is_numeric());
            }
            else if first.is_alphabetic(){
                return find_delimiter(s, |c| ! c.is_alphanumeric())  
            }
            else if is_valid_symbol(first){
                return find_delimiter(s, |c| ! is_valid_symbol(c));
            }
            else{
                return (" ", " ");
            }
        }
        None => {return (" ", " ");}
    
    }   
}
fn main() {
    println!("{}", get_first_token("17peepeepoo111=45").0);
}
