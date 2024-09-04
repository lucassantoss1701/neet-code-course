mod array;
mod stack;

fn main() {
    let is_valid = stack::valid_parentheses::is_valid("()".to_string());
    println!("{is_valid}");

    let is_valid = stack::valid_parentheses::is_valid("()[]{}".to_string());
    println!("{is_valid}");

    let is_valid = stack::valid_parentheses::is_valid("(]".to_string());
    println!("{is_valid}");

    let is_valid = stack::valid_parentheses::is_valid("([])".to_string());
    println!("{is_valid}");
}
