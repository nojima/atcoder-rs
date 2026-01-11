/*
    0123456
    0010010
     54 56
    0011101
     5  56 
    0000010
        56
    1111100
        5
    0000000
*/
use std::io::Read;

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let S: Vec<_> = input.trim().chars().collect();

    let mut k = S.len();
    for i in 0..(S.len()-1) {
        if S[i] != S[i+1] {
            let l = i+1;
            let r = S.len()-i-1;
            k = std::cmp::min(k, std::cmp::max(l, r));
        }
    }

    println!("{k}");

    Ok(())
}
