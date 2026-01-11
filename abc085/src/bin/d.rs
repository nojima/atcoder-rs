use std::io::Read;

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut words = input.split_ascii_whitespace();

    let N: usize = words.next().unwrap().parse()?;
    let mut H: usize = words.next().unwrap().parse()?;
    let mut a: Vec<usize> = Vec::with_capacity(N);
    let mut b: Vec<usize> = Vec::with_capacity(N);
    for _ in 0..N {
        a.push(words.next().unwrap().parse()?);
        b.push(words.next().unwrap().parse()?);
    }

    let max_a = *a.iter().max().unwrap();

    let mut n_atk = 0;

    // 攻撃の順番は関係ないので、先に刀を投げてから斬りつけるとしてもよい
    // まずは強い刀から順番に投げつけてみる
    b.sort_by(|a, b| b.cmp(a));
    for bi in b {
        if bi <= max_a { break }
        // b[i] を投げつける
        H = H.saturating_sub(bi);
        n_atk += 1;
        if H == 0 { break }
    }

    // 次に最も強い刀で死ぬまで切る
    n_atk += H.div_ceil(max_a);

    println!("{n_atk}");

    Ok(())
}
