use std::{collections::BTreeMap, io::Read};

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let re_spaces = regex::Regex::new(r"\s+")?;
    let mut words = re_spaces.split(&input);

    let N: usize = words.next().unwrap().parse()?;
    let K: usize = words.next().unwrap().parse()?;
    let A: Vec<i64> = words
        .take(N)
        .map(|a| a.parse())
        .collect::<Result<_, _>>()?;

    let mut freq: BTreeMap<i64, usize> = BTreeMap::new();
    for a in A {
        freq.entry(a)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    let mut freq_values: Vec<_> = freq.values().copied().collect();
    freq_values.sort();

    let end = freq.len().saturating_sub(K);
    let sum: usize = freq_values.iter().take(end).sum();
    println!("{}", sum);

    Ok(())
}
