
use std::fs;


#[derive(PartialEq, Eq, Debug)]
pub enum Type {
    Naughty,
    Nice
}

fn check_naughty_combos(input: &str) -> bool
{
    let naughty_combos = ["ab", "cd", "pq", "xy"];
    for combo in naughty_combos {
        if input.contains(combo) {
            return true;
        }
    }
    return false;
}

fn check_3_vowels(input: &str) -> bool {
    let mut vowel_cnt = 0;
    let vowels = "aeiou";
    for letter in input.chars() {
        if vowels.contains(letter){
            vowel_cnt += 1;
        }
        if vowel_cnt >= 3 {
            return true 
        }
    }
    return false;
}

fn check_double_letters(input: &str) -> bool {
    let mut last_letter = String::new();
    for letter in input.chars() {
        if last_letter.contains(letter) {
            return true;
        }
        last_letter = letter.to_string();
    }

    return false;
}

pub fn determin_type(input: &str) -> Type{

    //Contains naughty combos
    match check_naughty_combos(input) {
        true => return Type::Naughty,
        false => ()
    }

    //Check 3 vowels
    let vowel_check =  check_3_vowels(input);

    //Contains a double letter
    let double_check = check_double_letters(input);    

    if vowel_check && double_check {
        return Type::Nice;
    }

    return Type::Naughty;

}


pub fn check_list(file_path: &str) -> i32 {
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

    return nice_count;

}