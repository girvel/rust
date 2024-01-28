use std::rc::Rc;

#[derive(Debug, Clone)]
struct Container(i32);

fn main() {
    //let rc_string: Rc<str>;
    //{
    //    let string = String::from("Hello, ") + "world";
    //    rc_string = Rc::from(string.as_str());
    //}
    //println!("{}", rc_string);
    
    let rc_slice: Rc<[Container]>;
    {
        let slice: &[Container] = &[Container(42)];
        rc_slice = Rc::from(slice);

    }
    println!("{:?}", rc_slice);
}
