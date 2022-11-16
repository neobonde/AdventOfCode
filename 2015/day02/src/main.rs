use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[cfg(test)]
mod tests;


mod present;

fn get_presents_from_file(file_path: &str) -> Vec<present::Present> {
    
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    let mut presents: Vec<present::Present> = vec![];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut dimensions = line.split("x");
                let present = present::Present::new(
                    dimensions.next().unwrap().parse::<i32>().unwrap(),
                    dimensions.next().unwrap().parse::<i32>().unwrap(),
                    dimensions.next().unwrap().parse::<i32>().unwrap());
                presents.push(present);
            },
            Err(error) => panic!("Error importing file: {}",error)
        }
    }

    presents
}

fn main() {
    println!("Hello, world!");
    let presents = get_presents_from_file("input.txt");

    let mut total_wrapping_paper = 0;
    let mut total_ribbon = 0;

    for present in presents {
        total_wrapping_paper += present.wrapping_paper_area();
        total_ribbon += present.ribbon_length();
    }

    println!("Total wrapping paper: {}",total_wrapping_paper);
    println!("Total ribbon: {}",total_ribbon);

}
