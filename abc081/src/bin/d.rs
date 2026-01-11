use std::{io::Read, sync::LazyLock};

type BResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(non_snake_case)]
fn main() -> BResult<()> {
    let mut words = read_input()?;

    let N: usize = words.next().unwrap().parse()?;
    let a: Vec<i64> = words
        .take(N)
        .map(|x| x.parse())
        .collect::<Result<_, _>>()?;

    let mut answer: Vec<(usize, usize)> = Vec::new();

    if a.iter().all(|&x| x >= 0) {
        for i in 0..(N-1) {
            answer.push((i, i+1));
        }
    } else if a.iter().all(|&x| x <= 0) {
        for i in 0..(N-1) {
            answer.push((N-i-1, N-i-2));
        }
    } else {
        let (max_i, max) = max_element(&a); // max >= 0
        let (min_i, min) = min_element(&a); // min <= 0
        if max.abs() >= min.abs() {
            for i in 0..N {
                if i != max_i {
                    answer.push((max_i, i))
                }
            }
            for i in 0..(N-1) {
                answer.push((i, i+1));
            }
        } else {
            for i in 0..N {
                if i != min_i {
                    answer.push((min_i, i))
                }
            }
            for i in 0..(N-1) {
                answer.push((N-i-1, N-i-2));
            }
        }
    }

    println!("{}", answer.len());
    for (x, y) in answer {
        println!("{} {}", x+1, y+1);
    }

    Ok(())
}

fn read_input() -> BResult<regex::Split<'static, 'static>> {
    static RE_SPACES: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"\s+").unwrap());
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let words = RE_SPACES.split(input.leak());
    Ok(words)
}

fn max_element<T: Ord + Copy>(a: &[T]) -> (usize, T) {
    assert_ne!(a.len(), 0);
    let mut max_i = 0;
    for i in 1..a.len() {
        if a[i] > a[max_i] {
            max_i = i;
        }
    }
    (max_i, a[max_i])
}

fn min_element<T: Ord + Copy>(a: &[T]) -> (usize, T) {
    assert_ne!(a.len(), 0);
    let mut min_i = 0;
    for i in 1..a.len() {
        if a[i] < a[min_i] {
            min_i = i;
        }
    }
    (min_i, a[min_i])
}
