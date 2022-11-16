
use crate::present::Present;

use super::present;

#[test]
fn wrapping_paper_2x3x4(){
    let present = present::Present::new(2,3,4);
    let result = present.wrapping_paper_area();
    assert_eq!(result,58);
}

#[test]
fn wrapping_paper_1x1x10(){
    let present = present::Present::new(1,1,10);
    let result = present.wrapping_paper_area();
    assert_eq!(result,43);
}

#[test]
fn ribbon_2x3x4(){
    let present = present::Present::new(2,3,4);
    let result = present.ribbon_length();
    assert_eq!(result,34)
}

#[test]
fn ribbon_1x1x10(){
    let present = present::Present::new(1,1,10);
    let result = present.ribbon_length();
    assert_eq!(result,14)
}

// 1+1+26+26 = 54 + 26*1*29 = 808
#[test]
fn ribbon_26x1x29() 
{
    let present = present::Present::new(26,1,29);
    let result: i32 = present.ribbon_length();
    assert_eq!(result,808)
}
