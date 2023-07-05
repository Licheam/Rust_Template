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
struct BIT {
    b: Vec<i64>,
    n: usize,
}

impl BIT {
    pub fn new(n: usize) -> Self {
        Self {
            b: vec![0; n+1],
            n
        }
    }

    fn lowbit(x: usize) -> usize {
        x & (!x + 1)
    }

    pub fn change(&mut self, mut x: usize, y: i64) {
        let BIT { b, n } = self;
        while x <= *n {
            b[x] += y;
            x += BIT::lowbit(x);
        }
    }

    pub fn sum(&self, mut x: usize) -> i64 {
        let BIT { b, .. } = &self;
        let mut s: i64 = 0;
        while x > 0 {
            s += b[x];
            x -= BIT::lowbit(x);
        }
        s
    }

    pub fn build(&mut self, a: &Vec<i64>) {
        let BIT { b, n } = self;
        for i in 1..=*n {
            b[i] = a[i-1];
        }
        let mut x = 1;
        while x << 1 <= *n {
            for i in (x..=*n-x).step_by(x << 1) {
                b[i+x] += b[i];
            }
            x <<= 1;
        }
    }
}

fn solve(inp: &mut Parser) {
    let (n,m): (usize, usize) = (inp.read(), inp.read());
    let a: Vec<i64> = (0..n).map(|_| inp.read()).collect();
    let mut bit = BIT::new(n);
    bit.build(&a);
    for _ in 0..m {
        let op: usize = inp.read();
        match op {
            1 => {
                let (x, y): (usize, i64) = (inp.read(), inp.read());
                bit.change(x, y);
            }
            2 => {
                let (x,y): (usize, usize) = (inp.read(), inp.read());
                println!("{}", bit.sum(y) - bit.sum(x-1));
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