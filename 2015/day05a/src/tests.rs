
use crate::naughty_or_nice::determin_type;

use super::naughty_or_nice;

#[test]
fn nice_string(){
    let input = "ugknbfddgicrmopn";
    let answer = naughty_or_nice::Type::Nice;
    let res = determin_type(input);
    assert_eq!(res,answer);
}


#[test]
fn simple_nice_string(){
    let input = "aaa";
    let answer = naughty_or_nice::Type::Nice;
    let res = determin_type(input);
    assert_eq!(res,answer);
}


#[test]
fn no_double_letter(){
    let input = "jchzalrnumimnmhp";
    let answer = naughty_or_nice::Type::Naughty;
    let res = determin_type(input);
    assert_eq!(res,answer);
}


#[test]
fn contains_xy(){
    let input = "haegwjzuvuyypxyu";
    let answer = naughty_or_nice::Type::Naughty;
    let res = determin_type(input);
    assert_eq!(res,answer);
}

#[test]
fn contains_only_one_vowel(){
    let input = "dvszwmarrgswjxmb";
    let answer = naughty_or_nice::Type::Naughty;
    let res = determin_type(input);
    assert_eq!(res,answer);
}