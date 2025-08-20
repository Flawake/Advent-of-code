struct Present {
    length: u32,
    width: u32,
    heigth: u32,
}

impl Present {
    fn new(length: u32, width: u32, heigth: u32) -> Self {
        Self {
            length,
            width,
            heigth,
        }
    }

    fn calculate_wrapping_area(&self) -> u32 {
        let face1 = self.length * self.width;
        let face2 = self.width * self.heigth;
        let face3 = self.heigth * self.length;
        (2 * face1) + (2 * face2) + (2 * face3) + face1.min(face2).min(face3)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let presents = parse_input(input);
    let res = presents
        .iter()
        .fold(0, |val, present| val + present.calculate_wrapping_area());
    println!("{}", res);
}

fn parse_input(input: &'static str) -> Vec<Present> {
    input
        .lines()
        .map(|present| {
            let [l, w, h] = present
                .split('x')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();
            Present::new(l, w, h)
        })
        .collect()
}
