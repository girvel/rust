fn call_function(f: fn() -> ()) {
    println!("[DEBUG] Size: {}", std::mem::size_of_val(&f));
    f();
}

fn call_function_cheaply<F: FnOnce()>(f: F) {
    println!("[DEBUG] Size: {}", std::mem::size_of_val(&f));
    f();
}

fn say_something() {
    println!("Something");
}

fn main() {
    call_function(|| println!("Hello world!"));
    call_function_cheaply(|| println!("Hello again!"));

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();

    call_function_cheaply(if input_string.trim() == "something" { say_something } else { || println!("Nothing.") });
}
