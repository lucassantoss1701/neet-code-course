mod array;
mod stack;

fn main() {
    let mut my_stack = stack::base::Stack::new(10);

    my_stack.push(2);
    println!("{:?}", my_stack.values);

    my_stack.push(6);
    println!("{:?}", my_stack.values);

    my_stack.push(9);
    println!("{:?}", my_stack.values);

    my_stack.push(5);
    println!("{:?}", my_stack.values);

    my_stack.pop();
    println!("{:?}", my_stack.values);

    my_stack.push(2);
    println!("{:?}", my_stack.values);

    my_stack.push(8);
    println!("{:?}", my_stack.values);

    my_stack.pop();
    println!("{:?}", my_stack.values);
}
