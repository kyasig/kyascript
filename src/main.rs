#[allow(dead_code)]

fn is_valid_keyword(s:&str)->bool{
    return ["if","while","else","elif","endif","endwhile"].contains(&s);
}
fn is_valid_symbol(s:&str)->bool{
    return ["+","-","*","/",":="].contains(&s);
}
fn is_valid_token(s: &str)->bool{
   return s.len() == 0 || s.chars().all(char::is_alphanumeric) || is_valid_keyword(s) || is_valid_symbol(s);
}
fn main() {
    println!("{}", is_valid_token(":="));
}
