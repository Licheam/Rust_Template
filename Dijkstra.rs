macro_rules! scanln {
    ($($i:ident), +) => {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        $($i = iter.next().unwrap().parse().unwrap();)*
    }
}

use std::collections::*;
use std::cmp::*;
const INF: i64 = 0x3f3f3f3f3f3f3f3f;

fn dij(n: usize, s: usize, e: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let mut dis = vec![INF; n+1];
    let mut vis = vec![false; n+1];
    dis[s]=0;
    let mut q = BinaryHeap::from([Reverse((dis[s], s))]);
    while let Some(Reverse((_, u))) = q.pop() {
        if vis[u] { continue; }
        vis[u]=true;
        for &(v, w) in &e[u] {
            if !vis[v] && dis[v] > dis[u] + w {
                dis[v] = dis[u] + w;
                q.push(Reverse((dis[v], v)));
            }
        }
    }
    dis
}

fn solve() {
    let (n, m, s): (usize, usize, usize);
    scanln!(n, m, s);
    let mut e: Vec<Vec<(usize, i64)>> = vec![vec![]; n+1];
    for _ in 0..m {
        let (f, g, w): (usize, usize, i64);
        scanln!(f, g, w);
        e[f].push((g, w));
    }

    let dis = dij(n, s, &e);

    for i in 1..=n {
        print!("{}{}", dis[i], if i == n { '\n' } else { ' ' });
    }
}

fn main() {
    let t: usize = 1;
    for _cas in 1..=t {
        // print!("Case #{}: ", _cas);
        solve();
    }
}