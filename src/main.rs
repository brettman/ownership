fn main() {
    let s = String::from("hello world");
    let idx = first_word(&s);
    println!("{}", idx);
}

// return the index of the end of the first word
fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
