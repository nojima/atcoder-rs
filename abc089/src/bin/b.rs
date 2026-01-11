use std::io::Read;

#[allow(non_snake_case)]
fn main() -> Result<(), AnyError> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut words = input.split_ascii_whitespace();

    let a = words.next().unwrap();
    let b = words.next().unwrap();

    let n: i64 = (a.to_owned() + b).parse()?;

    let mut i = 0;
    let ok = loop {
        if i*i > n { break false }
        if i*i == n { break true }
        i += 1;
    };
    println!("{}", if ok { "Yes" } else { "No" });

    Ok(())
}

// A poor man's anyhow::Error
struct AnyError(Box<dyn std::error::Error + 'static>, std::backtrace::Backtrace);
impl std::fmt::Debug for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}\n=== Backtrace ===\n{}", self.0, self.1)
    }
}
impl<E: std::error::Error + 'static> From<E> for AnyError {
    fn from(error: E) -> Self {
        AnyError(Box::new(error), std::backtrace::Backtrace::capture())
    }
}
