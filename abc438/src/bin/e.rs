use std::io::Read;

#[derive(Debug, Clone, Copy)]
struct Jump {
    dst: usize,
    gain: usize,
}

impl Default for Jump {
    fn default() -> Self {
        Self { dst: 0, gain: 0 }
    }
}

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut words = input.split_ascii_whitespace();

    let N: usize = words.next().unwrap().parse()?;
    let Q: usize = words.next().unwrap().parse()?;

    let A: Vec<usize> = (&mut words)
        .take(N)
        .map(|a| a.parse::<usize>().unwrap() - 1)
        .collect();

    // jump[k][src]: 現在 src にいるときの、2^k ターン後の位置と獲得水量
    let mut jumps: Vec<Vec<Jump>> = vec![vec![Jump::default(); N]; 31];

    // k=0
    for src in 0..N {
        jumps[0][src] = Jump { dst: A[src], gain: src+1 }
    }

    // 配るDPを行う
    for k in 0..30 {
        for src in 0..N {
            let dst1 = jumps[k][src].dst;
            let dst2 = jumps[k][dst1].dst;
            let gain1 = jumps[k][src].gain;
            let gain2 = jumps[k][dst1].gain;
            jumps[k+1][src] = Jump {
                dst: dst2,
                gain: gain1 + gain2,
            };
        }
    }

    for _ in 0..Q {
        let mut t = words.next().unwrap().parse::<usize>()?;
        let mut b = words.next().unwrap().parse::<usize>()? - 1;

        let mut k = 0;
        let mut gain = 0;
        while t > 0 {
            if t % 2 == 1 {
                let j = jumps[k][b];
                b = j.dst;
                gain += j.gain;
            }
            t /= 2;
            k += 1;
        }

        println!("{gain}");
    }

    Ok(())
}
