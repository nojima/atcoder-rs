use std::io::Read;

#[allow(non_snake_case)]
fn main() -> Result<(), AnyError> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut words = input.split_ascii_whitespace();

    let N: usize = words.next().unwrap().parse()?;
    let K: usize = words.next().unwrap().parse()?;
    let mut requests = vec![vec![0usize; 4*K]; 4*K];
    for _ in 0..N {
        let mut xi = words.next().unwrap().parse::<usize>()?;
        let mut yi = words.next().unwrap().parse::<usize>()?;
        let ci = words.next().unwrap();
        // (xi, yi) が白である ⇔ (xi+K, yi) が黒である
        // よって、白リクエストは黒リクエストに変換できる
        if ci == "W" {
            xi += K;
        }
        // 周期性により、mod 2K で考えてよい
        xi %= 2*K;
        yi %= 2*K;
        requests[yi][xi] += 1;
    }
    // 全探索の際に [0, 2K) の区間からはみ出てもよいようにするために
    // [2K, 4K) にも値を入れる
    for y in 0..(4*K) {
        for x in 0..(4*K) {
            if y < (2*K) && x < (2*K) { continue }
            requests[y][x] = requests[y % (2*K)][x % (2*K)];
        }
    }

    let cumsum = CumSum2D::new(requests);

    let mut max_count = 0;
    for offset_y in 0..(2*K) {
        for offset_x in 0..(2*K) {
            let count =
                cumsum.sum(offset_y,     offset_x,     K, K) +
                cumsum.sum(offset_y + K, offset_x + K, K, K);
            //eprintln!("y={offset_y}, x={offset_x}, count={count}");
            max_count = std::cmp::max(max_count, count);
        }
    }

    println!("{max_count}");

    Ok(())
}

struct CumSum2D(Vec<Vec<usize>>);

impl CumSum2D {
    fn new(mut v: Vec<Vec<usize>>) -> Self {
        for y in 0..v.len() {
            for x in 0..v[y].len() {
                if y == 0 && x == 0 {
                    // do nothing
                } else if y == 0 {
                    v[y][x] += v[y][x-1];
                } else if x == 0 {
                    v[y][x] += v[y-1][x];
                } else {
                    v[y][x] += v[y-1][x] + v[y][x-1] - v[y-1][x-1];
                }
            }
        }
        Self(v)
    }

    fn sum(&self, y: usize, x: usize, h: usize, w: usize) -> usize {
        let v = &self.0;
        if y == 0 && x == 0 {
            v[h-1][w-1]
        } else if y == 0 {
            v[h-1][x+w-1] - v[h-1][x-1]
        } else if x == 0 {
            v[y+h-1][w-1] - v[y-1][w-1]
        } else {
            v[y+h-1][x+w-1]
                + v[y-1][x-1]
                - v[y+h-1][x-1]
                - v[y-1][x+w-1]
        }
    }
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
