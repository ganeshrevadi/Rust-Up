fn main() {
    let is_male:bool = true;
    let mut is_above_18:bool = true;

    is_above_18 = false;

    if is_male{
        println!("This person is male.");
    }else{
        println!("This person is not male.");
    }

    if is_male && is_above_18 {
        println!("This person is a male and above 18.");
    } else {
        println!("This person is either not male or not above 18.");
    }
    let sum = add(5, 10);
    println!("The sum is: {}", sum);
}

fn add(a:i32, b:i32) -> i32 {
    return a + b
}
