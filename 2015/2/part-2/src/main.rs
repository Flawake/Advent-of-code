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

    fn calculate_ribbon_length(&self) -> u32 {
        let l = self.length;
        let w = self.width;
        let h = self.heigth;

        let smallest = l.min(w.min(h));
        let largest = l.max(w.max(h));
        let middle = l + w + h - smallest - largest;

        smallest + smallest + middle + middle + (self.length * self.width * self.heigth)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let presents = parse_input(input);
    let res = presents
        .iter()
        .fold(0, |val, present| val + present.calculate_ribbon_length());
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
