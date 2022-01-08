fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s); // s moves its value to the function and is no longer valid

    let x = 5;      // x comes into scope
    makes_copy(x);  // i32 is `copy` so it's still ok to use x after this call

}

fn takes_ownership(some_string: String){  // some_string comes into scope
    println!("{}", some_string); 
} // Here, some_string goes out of scope and `drop` is called.  The backing memory is freed

fn makes_copy(i: i32){  // i comes into scope
    println!("{}", i);
} // i goes out of scope, nothing special happens as this is a stack value
