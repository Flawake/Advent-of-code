use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let ans = calculate(&input);
    //let ans = check_safe_count_2(data);
    println!("{}", ans);
    Ok(())
}

fn calculate(input: &String) -> u32 {
    let mut count = 0;
    let bytes_per_line = input.char_indices().find(|(_, x)| *x == '\n').unwrap().0 + 1;
    let line_count = input.lines().count();

    let input_chars: Vec<char> = input.chars().collect();

    for x in 0..bytes_per_line - 2 {
        for y in 0..line_count - 2 {
            if input_chars[bytes_per_line * y + x] == 'M' {
                if input_chars[bytes_per_line * (y + 1) + x + 1] == 'A' {
                    if input_chars[bytes_per_line * (y + 2) + x + 2] == 'S' {

                        if input_chars[bytes_per_line * (y + 2) + x] == 'M' {
                            if input_chars[bytes_per_line * (y + 1) + x + 1] == 'A' {
                                if input_chars[bytes_per_line * (y) + x + 2] == 'S' {
                                    count += 1;
                                }
                            }
                        }

                        if input_chars[bytes_per_line * (y + 2) + x] == 'S' {
                            if input_chars[bytes_per_line * (y + 1) + x + 1] == 'A' {
                                if input_chars[bytes_per_line * (y) + x + 2] == 'M' {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }

            if input_chars[bytes_per_line * y + x] == 'S' {
                if input_chars[bytes_per_line * (y + 1) + x + 1] == 'A' {
                    if input_chars[bytes_per_line * (y + 2) + x + 2] == 'M' {

                        if input_chars[bytes_per_line * (y + 2) + x] == 'M' {
                            if input_chars[bytes_per_line * (y + 1) + x + 1] == 'A' {
                                if input_chars[bytes_per_line * (y) + x + 2] == 'S' {
                                    count += 1;
                                }
                            }
                        }

                        if input_chars[bytes_per_line * (y + 2) + x] == 'S' {
                            if input_chars[bytes_per_line * (y + 1) + x + 1] == 'A' {
                                if input_chars[bytes_per_line * (y) + x + 2] == 'M' {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}