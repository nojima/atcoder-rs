use std::io::Read;

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut words = input.split_ascii_whitespace();

    let X: i64 = words.next().unwrap().parse()?;
    let Y: i64 = words.next().unwrap().parse()?;

    let mut len = 1;
    let mut x = X;
    loop {
        x = x.saturating_mul(2);
        if x > Y { break }
        len += 1;
    }

    println!("{len}");

    Ok(())
}
