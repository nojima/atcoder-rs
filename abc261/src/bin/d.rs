use std::{cmp::max, io::Read};

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut words = input.split_ascii_whitespace();

    let N: usize = words.next().unwrap().parse()?;
    let M: usize = words.next().unwrap().parse()?;

    let mut X: Vec<i64> = vec![0; N+1];
    for i in 0..N {
        let xi: i64 = words.next().unwrap().parse()?;
        X[i+1] = xi;
    }
    let mut bonus: Vec<i64> = vec![0; N+1];
    for _ in 0..M {
        let ci: usize = words.next().unwrap().parse()?;
        let yi: i64 = words.next().unwrap().parse()?;
        bonus[ci] = yi;
    }

    // dp[n][c]: コイントスをn回行い、最終的にカウンタの値がcであるときの最大獲得金額
    let mut dp = vec![vec![i64::MIN; N+1]; N+1];
    // 配るDPを行う
    dp[0][0] = 0;
    for n in 0..N {
        for c in 0..N {
            // 表
            dp[n+1][c+1] = max(dp[n+1][c+1], dp[n][c] + X[n+1] + bonus[c+1]);
            // 裏
            dp[n+1][0] = max(dp[n+1][0], dp[n][c]);
        }
    }

    let answer = dp[N].iter().max().unwrap();
    println!("{answer}");

    Ok(())
}
