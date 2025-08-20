use md5;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", calculate(input));
}

fn hash_complies(hash: [u8; 16], zero_count: usize) -> bool {
    (0..zero_count).all(|i| {
        let byte = hash[i / 2];
        let half = if i % 2 == 0 { byte >> 4 } else { byte & 0x0F };
        half == 0
    })
}

fn calculate(input: &'static str) -> u32 {
    let mut iteration = 0;
    loop {
        let data = format!("{}{}", input, iteration.to_string());
        let hash = md5::compute(data);
        if hash_complies(hash.into(), 5) {
            return iteration;
        }
        iteration += 1;
    }
}