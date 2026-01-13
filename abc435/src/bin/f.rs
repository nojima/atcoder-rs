use std::{cmp::max, io::Read};

const INF: i64 = 1_000_000_000_000_000;

#[allow(non_snake_case, clippy::needless_range_loop)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut words = input.split_ascii_whitespace();

    let N = words.next().unwrap().parse::<usize>()?;
    let P = words
        .take(N)
        .map(|pi| pi.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    // index[i]: 高さがi番目(昇順)のタワーの位置
    let mut index = vec![0; N];
    for i in 0..N {
        index[P[i]] = i;
    }

    // st_h: 高さの最大値を返す SegmentTree
    let mut st_h = SegmentTree::new(N, 0usize, max);
    for i in 0..N {
        st_h.set(i, P[i]);
    }

    // dp[x]: 座標xのタワーにいるとき、最大で何点取れるか
    let mut dp = vec![0; N];
    // st_l: {dp[x] - x} の最大値を返す SegmentTree
    let mut st_l = SegmentTree::new(N, -INF, max);
    // st_r: {dp[x] + x} の最大値を返す SegmentTree
    let mut st_r = SegmentTree::new(N, -INF, max);

    // 低いタワーから順に dp を埋めていく

    // h=0
    let x = index[0];
    st_l.set(x, -(x as i64));
    st_r.set(x, x as i64);

    // h>0
    for h in 1..N {
        let x = index[h];
        // どこまで左右に動けるか求める
        let l = bisect_min(0, x,   |j| st_h.prod(j, x) < h);
        let r = bisect_max(x, N-1, |j| st_h.prod(x+1, j) < h);
        // 最大スコアを計算
        let l_max = if l >= x { 0 } else { st_l.prod(l, x) + x as i64 };
        let r_max = if r <= x { 0 } else { st_r.prod(x+1, r+1) - x as i64 };
        dp[x] = max(max(l_max, r_max), 0);
        st_l.set(x, dp[x] - x as i64);
        st_r.set(x, dp[x] + x as i64);
        //eprintln!("h={h}, x={x}, l={l}, r={r}, l_max={l_max}, r_max={r_max}, dp={}", dp[x]);
    }

    println!("{}", dp[index[N-1]]);

    Ok(())
}

pub struct SegmentTree<T, Product> {
    n: usize,
    data: Vec<T>,
    identity: T,
    product: Product,
}

impl<T, Product> SegmentTree<T, Product>
where
    T: Copy,
    Product: Fn(T, T) -> T,
{
    pub fn new(size: usize, identity: T, product: Product) -> Self {
        let n = bit_ceil(size);
        Self {
            n,
            data: vec![identity; 2*n+1],
            identity,
            product,
        }
    }

    pub fn get(&self, i: usize) -> T { self.data[self.n+i-1] }
    pub fn set(&mut self, i: usize, x: T) { self.set_(i, x, 0, 0, self.n) }

    // 区間 [l, r) のすべての要素の product による積を返す
    pub fn prod(&self, l: usize, r: usize) -> T { self.prod_(l, r, 0, 0, self.n) }

    fn set_(&mut self, i: usize, x: T, v: usize, nl: usize, nr: usize) {
        debug_assert!(nl <= i && i < nr);
        if nl == i && nr == i+1 {
            self.data[v] = x;
        } else {
            let nm = (nl + nr) / 2;
            if i < nm { self.set_(i, x, 2*v+1, nl, nm) }
            else      { self.set_(i, x, 2*v+2, nm, nr) }
            self.data[v] = (self.product)(self.data[2*v+1], self.data[2*v+2]);
        }
    }

    fn prod_(&self, l: usize, r: usize, v: usize, nl: usize, nr: usize) -> T {
        if r <= nl || nr <= l { return self.identity }
        if l <= nl && nr <= r { return self.data[v] }
        let nm = (nl + nr) / 2;
        let x1 = self.prod_(l, r, 2*v+1, nl, nm);
        let x2 = self.prod_(l, r, 2*v+2, nm, nr);
        (self.product)(x1, x2)
    }
}

fn bit_ceil(n: usize) -> usize {
    let mut ret = 1;
    while ret < n { ret *= 2 }
    ret
}

// pred(x) を区間[a, b)で定義された単調増加なbool値関数とする。
// このとき pred(x) = true となるような最小の x を返す。
// そのような x が存在しないときは b を返す。
pub fn bisect_min(a: usize, b: usize, pred: impl Fn(usize) -> bool) -> usize {
    // 不変条件: i<a ⇒ pred(i)=false, b<=i ⇒ pred(i)=true
    if a == b { return b }
    let m = a + (b - a) / 2;
    if pred(m) {
        bisect_min(a, m, pred)
    } else {
        bisect_min(m+1, b, pred)
    }
}

// pred(x) を区間(a, b]で定義された単調減少なbool値関数とする。
// (定義域に注意!!)
// このとき pred(x) = true となるような最大の x を返す。
// そのような x が存在しないときは a を返す。
pub fn bisect_max(a: usize, b: usize, pred: impl Fn(usize) -> bool) -> usize {
    // 不変条件: i<=a ⇒ pred(i)=true, b<i ⇒ pred(i)=false
    if a == b { return a }
    let m = a + (b - a).div_ceil(2);
    if pred(m) {
        bisect_max(m, b, pred)
    } else {
        bisect_max(a, m-1, pred)
    }
}
