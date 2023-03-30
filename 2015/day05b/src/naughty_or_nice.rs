
use std::{fs, iter::Enumerate};


#[derive(PartialEq, Eq, Debug)]
pub enum Type {
    Naughty,
    Nice
}

fn check_letter_pair(input: &str) -> bool{

    let mut pairs = vec![];
    for i in 2..input.len()+1 {
        pairs.push(&input[i-2..i]);
    }

    // dbg!(&pairs);
    let mut repeats = 0;
    for (i, pair1) in pairs.iter().enumerate() {
        repeats = 0; 
        for (j, pair2) in pairs[i..].iter().enumerate() {
            if pair1.contains(pair2) {
                repeats +=1;
                // dbg!(repeats);
            }
            if repeats >= 2{
                if i != j+i-1 {
                    // dbg!("match pair");
                    // dbg!(repeats,i,j, pair1, pair2);
                    // dbg!("match pair");
                    return true;
                }
                repeats = 0;
            }
        }
    }

    return false

}

fn check_letter_repeat(input: &str) -> bool{
    
    // dbg!(input);
    for (i, letter1) in input[2..].chars().enumerate() {
        let letter2 = input.chars().nth(i).unwrap();
        // dbg!(i,letter1, letter2);
        if letter1 == letter2 
        {
            // dbg!("match",);
            return true
        }
    }
    
    false
}


pub fn determin_type(input: &str) -> Type{

    let check_pair = check_letter_pair(input);
    // dbg!(check_pair);

    let check_repeat = check_letter_repeat(input);
    // dbg!(check_repeat);

    if check_pair && check_repeat == true {
        return Type::Nice
    }
    
    return Type::Naughty;

}


pub fn check_list(file_path: &str) -> (i32,i32) {
    let lines = fs::read_to_string(file_path).unwrap();
    let list:Vec<&str> = lines.lines().collect();

    let mut nice_count = 0;
    let mut naughty_count = 0;

    for item in list {
        match determin_type(item) {
            Type::Nice => nice_count += 1,
            Type::Naughty => naughty_count +=1,
        }
    }

    return (nice_count,naughty_count);

}