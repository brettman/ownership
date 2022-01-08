fn main() {
    example_one_tuples();
    example_two_reference();
    example_three_mutable_references();
}

fn example_one_tuples(){

    let s1 = String::from ("hello");
    let (s2, len) = calculate_length(s1);
    // can't use s1, because it goes out of scope when we move it to
    //  the function to calc length. 
    // That function has to return a tuple with the original 
    //  value and the length, to do the following:
    println!("the length of '{}' is {}.", s2, len);
}

fn example_two_reference(){
    let s1 = String::from("hello, again");
    let len = calculate_length1(&s1);
    println!("the length of '{}' is {}.", s1, len);
}

fn example_three_mutable_references(){
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String){
    s.push_str(", world!");
}

fn calculate_length(s: String) -> (String, usize){
    let l = s.len();

    (s, l)      // notice lack of semi-colon? this means it is the return statement; 
}

fn calculate_length1(s: &String) -> usize{
    s.len()
}