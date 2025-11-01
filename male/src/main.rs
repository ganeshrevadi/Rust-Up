fn main() {
    let is_male:bool = true;
    let is_above_18:bool = true;

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

}
