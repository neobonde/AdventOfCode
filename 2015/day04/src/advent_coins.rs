use md5;


pub fn hash(input: &str, answer: i32){
    let context = format!("{input}{answer}");
    let digest = md5::compute(&context);
    let hash = format!("{:x}",digest);
    println!("{context} \t md5: 0x{hash}");
    let substr = hash.get(0..5).unwrap_or(&hash);
    let test = substr == "00000";
    println!("{substr}, {test}");
}

pub fn mine(input: &str, comparator: &str) -> i32{
    
    let mut answer = -1;
    let mut hash = String::new();
    while hash.get(0..comparator.len()).unwrap_or(&hash) != comparator{
        answer += 1;
        let context = format!("{input}{answer}");
        let digest = md5::compute(context);
        hash = format!("{:x}",digest);
    }
    
    return answer;
}