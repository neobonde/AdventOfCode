
use crate::naughty_or_nice::determin_type;

use super::naughty_or_nice;


#[test]
fn test_nice_pair(){
    let input = "qjhvhtzxzqqjkmpb";
    let answer = naughty_or_nice::Type::Nice;
    let res = determin_type(input);
    assert_eq!(answer,res);
}

#[test]
fn test_nice_rule_overlap(){
    let input = "xxyxx";
    let answer = naughty_or_nice::Type::Nice;
    let res = determin_type(input);
    assert_eq!(answer,res);
}

#[test]
fn test_naughty_no_repeat(){
    let input = "uurcxstgmygtbstg";
    let answer = naughty_or_nice::Type::Naughty;
    let res = determin_type(input);
    assert_eq!(answer,res);
}

#[test]
fn test_naughty_no_pair(){
    let input = "ieodomkazucvgmuy";
    let answer = naughty_or_nice::Type::Naughty;
    let res = determin_type(input);
    assert_eq!(answer,res);
}

#[test]
fn test_1(){
    let input = "punnnfyyufkpqilx"; 
    let answer = naughty_or_nice::Type::Naughty;
    let res = determin_type(input);
    assert_eq!(answer,res);
}