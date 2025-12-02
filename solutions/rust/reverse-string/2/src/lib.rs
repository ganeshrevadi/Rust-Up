pub fn reverse(input: &str) -> String {
    let mut chars : Vec<char> = input.chars().collect(); 
     let x = chars.len();
    
    for i in 0..chars.len() / 2{
        chars.swap(i, x - 1 - i);
    }
    let my_string: String = chars.into_iter().collect();
    return my_string;
}
