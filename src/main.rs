fn main() {
    let s = String::from("hello world");
    let idx = first_word(&s);
    println!("{}", idx);
}

// return the index of the end of the first word
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    
    // iter returns each element of array in a collection
    // enumerate() wraps each element as a tuple (index, value)
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
           return &s[0..i];
        }
    };
    &s[..]
}
