mod cp {
    use std::io::prelude::*;
    pub fn stdin_to_string() -> String {
        let mut content = String::new();
        let stdin = std::io::stdin();
        let mut rd = stdin.lock();
        rd.read_to_string(&mut content).unwrap();
        return content;
    }

    pub struct Parser<'a> {
        tokens: std::str::SplitWhitespace<'a>,
    }

    impl<'a> Parser<'a> {
        pub fn new(text: &'a str) -> Self {
            Self {
                tokens: text.split_whitespace(),
            }
        }

        pub fn read_or_eof<T: std::str::FromStr>(&mut self) -> Option<T> {
            self.next().map(|s| match s.parse() {
                Ok(x) => x,
                Err(_) => panic!("cannot parse {:?}", s),
            })
        }

        pub fn read<T: std::str::FromStr>(&mut self) -> T {
            self.read_or_eof::<T>().expect("unexpected end-of-file")
        }
    }

    impl<'a> Iterator for Parser<'a> {
        type Item = &'a str;
        fn next(&mut self) -> Option<&'a str> {
            self.tokens.next()
        }
    }
}

use cp::*;
use std::*;

#[derive(Debug)]
struct SGT {
    sum: Vec<i64>,
    tag: Vec<i64>,
}

impl SGT {
    pub fn new(n: usize) -> Self {
        Self {
            sum: vec![0; n<<2],
            tag: vec![0; n<<2],
        }
    }

    fn pushup(&mut self, x: usize) {
        let SGT { sum, .. } = self;
        sum[x] = sum[x<<1] + sum[x<<1|1];
    }

    fn pushdown(&mut self, x: usize, l: usize, r: usize) {
        let SGT { sum, tag } = self;
        let m = (l+r)>>1;
        sum[x<<1] += tag[x]*(m-l+1) as i64;
        tag[x<<1] += tag[x];
        sum[x<<1|1] += tag[x]*(r-m) as i64;
        tag[x<<1|1] += tag[x];
        tag[x] = 0;
    }

    pub fn build(&mut self, x: usize, l: usize, r: usize, a: &Vec<i64>) {
        let SGT { sum, .. } = self;
        if l == r { sum[x] = a[l-1] }
        else {
            let m = (l+r)>>1;
            self.build(x<<1, l, m, a);
            self.build(x<<1|1, m+1, r, a);
            self.pushup(x);
        }
    }

    pub fn modify(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize, del: i64) {
        if ql <= l && r <= qr {
            let SGT { sum, tag } = self;
            sum[x] += del*(r-l+1) as i64;
            tag[x] += del;
        } else {
            self.pushdown(x, l, r);
            let m = (l+r)>>1;
            if ql <= m { self.modify(x<<1, l, m, ql, qr, del) }
            if m < qr { self.modify(x<<1|1, m+1, r, ql, qr, del) }
            // let SGT { sum, tag } = self;
            // sum[x] = tag[x]*(r-l+1) as i64 + sum[x<<1] + sum[x<<1|1];
            self.pushup(x);
        }
    }

    pub fn query(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if ql <= l && r <= qr { self.sum[x] }
        else {
            self.pushdown(x, l, r);
            let m = (l+r)>>1;
            (if ql <= m { self.query(x<<1, l, m, ql, qr) } else { 0 })
            + (if m < qr { self.query(x<<1|1, m+1, r, ql, qr) } else { 0 })
            // + tag[x] * (cmp::min(qr,r)-cmp::max(ql,l)+1) as i64
        }
    }
}

fn solve(inp: &mut Parser) {
    let (n,m): (usize, usize) = (inp.read(), inp.read());
    let a: Vec<i64> = (0..n).map(|_| inp.read()).collect();
    let mut sgt = SGT::new(n);
    sgt.build(1,1,n,&a);
    for _ in 0..m {
        let op: usize = inp.read();
        match op {
            1 => {
                let (l, r, k): (usize, usize, i64) = (inp.read(), inp.read(), inp.read());
                sgt.modify(1,1,n,l,r,k);
            }
            2 => {
                let (l, r): (usize, usize) = (inp.read(), inp.read());
                println!("{}", sgt.query(1,1,n,l,r));
            }
            _ => {}
        }
    }
}

fn main() {
    let content = stdin_to_string();
    let mut inp = Parser::new(&content);
    let t: usize = 1;
    for _cas in 1..=t {
        // print!("Case #{}: ", _cas);
        solve(&mut inp);
    }
}