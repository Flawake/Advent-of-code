use std::error::Error;
use std::fs;

#[derive(Debug)]
struct DiskFile {
    file_id: u32,
    file_size: u32,
    empty_space: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;

    let disk_files = string_to_files(&input);
    let disk: Vec<Option<u32>> = files_to_disk(&disk_files);
    let ans = defragment_disk(disk);
    println!("{}", ans);
    Ok(())
}

fn string_to_files(input: &String) -> Vec<DiskFile> {
    let mut files = vec![];

    let mut file_id = 0;
    let mut chars = input.chars();

    for _ in (0..input.len()).step_by(2) {
        let mut size = 0;
        let mut empty = 0;

        if let Some(c) = chars.nth(0) {
            if let Some(v) = c.to_digit(10) {
                size = v;
            }
        }
        if let Some(c) = chars.nth(0) {
            if let Some(v) = c.to_digit(10) {
                empty = v;
            }
        }

        files.push(DiskFile {
            file_id: file_id,
            file_size: size,
            empty_space: empty,
        });
        file_id += 1;
    }

    files
}

fn files_to_disk(files: &Vec<DiskFile>) -> Vec<Option<u32>> {
    let mut disk: Vec<Option<u32>> = vec![];

    for file in files {
        for _ in 0..file.file_size {
            disk.push(Some(file.file_id));
        }
        for _ in 0..file.empty_space {
            disk.push(None);
        }
    }

    println!("{:?}", disk);
    disk
}

fn defragment_disk(disk: Vec<Option<u32>>) -> u64 {
    let mut defragmented_disk: Vec<Option<u32>> = vec![];
    let mut last_read_index = disk.len() - 1;
    
    for (i, c) in disk.iter().enumerate() {
        if let Some(val) = c {
            defragmented_disk.push(Some(*val));
        } else {
            while disk[last_read_index] == None {
                last_read_index -= 1;
            }
            if last_read_index <= i {
                break;
            }
            defragmented_disk.push(disk[last_read_index]);
            last_read_index -= 1;
        }
        if last_read_index == i {
            break;
        }
    }

    //println!("{:?}", defragmented_files);
    let checksum = calculate_checksum(defragmented_disk);
    checksum
}


fn calculate_checksum(defragmented_disk: Vec<Option<u32>>) -> u64 {
    println!("{:?}", defragmented_disk);
    let mut checksum = 0;
    for (i, c) in defragmented_disk.into_iter().enumerate() {
        if let Some(val) = c {
            checksum += val as u64 * i as u64;
        } else {
           break;
        }
    }

    println!("{:?}", checksum);

    checksum
}
