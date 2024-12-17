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
    disk
}

fn defragment_disk(disk: Vec<Option<u32>>) -> u64 {
    println!("{:?}", disk);
    let mut defragmented_disk = disk.clone();
    let mut i = disk.len();

    while i > 0 {
        i -= 1;

        let c = defragmented_disk[i];
        if c == None {
            continue;
        }

        let checking_file_id = c.unwrap();

        //Calculate length of current file
        let mut file_length = 0;
        for j in (0..i + 1).rev() {
            if let Some(id) = defragmented_disk[j] {
                if id == checking_file_id {
                    file_length += 1;
                } else {
                    break;
                }
            }
        }

        //Find the first part of free space that is big enough to hold this file
        let mut free_space = 0;
        let mut free_space_start = 0;
        for j in 0..i {
            if defragmented_disk[j] == None {
                free_space += 1;
            } else {
                free_space = 0;
            }
            if free_space == file_length {
                free_space_start = j - free_space + 1;
                break;
            }
        }

        if free_space != 0 {
            for j in 0..file_length {
                defragmented_disk[free_space_start + j] = Some(checking_file_id);
                defragmented_disk[i - j] = None;
            }
        }

        if file_length > 1 {
            i = i.saturating_sub(file_length - 1);
        }

    }

    println!("{:?}", defragmented_disk);
    let checksum = calculate_checksum(defragmented_disk);
    checksum
}

fn calculate_checksum(defragmented_disk: Vec<Option<u32>>) -> u64 {
    let mut checksum = 0;
    for (i, c) in defragmented_disk.into_iter().enumerate() {
        if let Some(val) = c {
            checksum += val as u64 * i as u64;
        }
    }

    checksum
}
