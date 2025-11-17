fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    update_word(s2);
    println!("{}", s2);
    println!("{}", s1);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}
