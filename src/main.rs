use std::rc::Rc;

fn main() {
    let rc_string: Rc<str>;
    {
        let string = String::from("Hello, ") + "world";
        rc_string = Rc::from(string.as_str());
    }
    println!("{}", rc_string);
}
