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
        for y in 0..line_count - 3 {
            if input_chars[bytes_per_line * y + x] == 'X' {
                if input_chars[bytes_per_line * (y + 1) + x] == 'M' {
                    if input_chars[bytes_per_line * (y + 2) + x] == 'A' {
                        if input_chars[bytes_per_line * (y + 3) + x] == 'S' {
                            count += 1;
                        }
                    }
                }

                if input_chars[bytes_per_line * (y + 1) + x + 1] == 'M' {
                    if input_chars[bytes_per_line * (y + 2) + x + 2] == 'A' {
                        if input_chars[bytes_per_line * (y + 3) + x + 3] == 'S' {
                            count += 1;
                        }
                    }
                }

                if input_chars[bytes_per_line * (y + 1) + x - 1] == 'M' {
                    if input_chars[bytes_per_line * (y + 2) + x - 2] == 'A' {
                        if input_chars[bytes_per_line * (y + 3) + x - 3] == 'S' {
                            count += 1;
                        }
                    }
                }
            }
            
            else if input_chars[bytes_per_line * y + x] == 'S' {
                if input_chars[bytes_per_line * (y + 1) + x] == 'A' {
                    if input_chars[bytes_per_line * (y + 2) + x] == 'M' {
                        if input_chars[bytes_per_line * (y + 3) + x] == 'X' {
                            count += 1;
                        }
                    }
                }

                if input_chars[bytes_per_line * (y + 1) + x + 1] == 'A' {
                    if input_chars[bytes_per_line * (y + 2) + x + 2] == 'M' {
                        if input_chars[bytes_per_line * (y + 3) + x + 3] == 'X' {
                            count += 1;
                        }
                    }
                }

                if input_chars[bytes_per_line * (y + 1) + x - 1] == 'A' {
                    if input_chars[bytes_per_line * (y + 2) + x - 2] == 'M' {
                        if input_chars[bytes_per_line * (y + 3) + x - 3] == 'X' {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count += input.chars().collect::<Vec<_>>().windows(4).filter(|x| x[0] == 'X' && x[1] == 'M' && x[2] == 'A' && x[3] == 'S').count() as u32;
    count += input.chars().collect::<Vec<_>>().windows(4).filter(|x| x[0] == 'S' && x[1] == 'A' && x[2] == 'M' && x[3] == 'X').count() as u32;

    count
}