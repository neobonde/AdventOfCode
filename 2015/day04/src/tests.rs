
use super::advent_coins;

#[test]
fn test1()
{
    let input = "abcdef";
    let answer = 609043;
    let res = advent_coins::mine(input, "00000");
    assert_eq!(res,answer);
}


#[test]
fn test2()
{
    let input = "pqrstuv";
    let answer = 1048970;
    let res = advent_coins::mine(input, "00000");
    assert_eq!(res,answer);
}


