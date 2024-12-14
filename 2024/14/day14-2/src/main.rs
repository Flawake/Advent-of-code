use std::error::Error;
use std::{fs, vec};
use image::GrayImage;
use rayon::prelude::*;

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;
    let parsed_input = parse_input(&input);
    let ans = calculate(parsed_input);
    Ok(())
}

fn parse_input(input: &String) -> Vec<Robot> {
    input.lines().into_iter().map(|s| {
        let v: Vec<&str> = s.split_whitespace().collect();
        let v1: Vec<&str> = v[0].split(",").collect();
        let v2: Vec<&str> = v[1].split(",").collect();
        Robot {
            pos: (v1[0][2..].trim().parse::<i32>().unwrap(), v1[1].trim().parse::<i32>().unwrap()),
            velocity: (v2[0][2..].trim().parse::<i32>().unwrap(), v2[1].trim().parse::<i32>().unwrap()),
        }
    }).collect()
}

fn calculate(robots: Vec<Robot>) {
    (0..10000).into_par_iter().for_each( |seconds|{
        let mut map= vec![vec![0;101];104];
        robots.iter().for_each(|robot| {
            let x = ((robot.pos.0 + (robot.velocity.0 * seconds)) % 101 + 101) % 101;
            let y = ((robot.pos.1 + (robot.velocity.1 * seconds)) % 103 + 103) % 103;
            map[y as usize][x as usize] = map[y as usize][x as usize] + 1;
            vec_to_pic(&map, seconds);
        });
        for row in map.iter_mut() {
            row.fill(0);
        }
    }) 
}

fn vec_to_pic(image_data: &Vec<Vec<u32>>, senconds: i32) {
    let width = image_data[0].len() as u32;
    let height = image_data.len() as u32;

    let mut buffer = Vec::with_capacity((width * height) as usize);

    for row in image_data {
        buffer.extend(row.into_iter().map(|&val| if val == 0 { 0u8 } else { 255u8 }));
    }

    let img = GrayImage::from_raw(width, height, buffer).expect("Failed to create image");

    let filename = format!("output/ output{}.png", senconds);
    img.save(filename).expect("Failed to save the image");

}