// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    {
        // let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    } // result 和 string1和string2 活得一样长，如果string2定义在{}中，则result生命周期在这一行结束
    println!("The longest string is '{}'", result);
}
