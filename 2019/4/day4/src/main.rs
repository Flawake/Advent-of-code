fn main() {
    let min: i64 = 273025;
    let max: i64 = 767253;
    let mut current_num = min;

    let mut total_valid: i64 = 0;
    
    while current_num < max {
        if is_valid(current_num) {
            total_valid += 1;
        }
        current_num += 1;
    }
    println!("There were a total of {} valid numbers in the given range", total_valid);
}

fn is_valid(num: i64) -> bool {
    let str_num = num.to_string();
    let mut adjesant = false;
    for x in 0..str_num.len() - 1{
        let first_char = str_num.chars().nth(x).unwrap();
        let next_char = str_num.chars().nth(x + 1).unwrap();
        if first_char == next_char {
            adjesant = true;
        }
        else if next_char < first_char {
            return false;
        }
    }
    adjesant
}
