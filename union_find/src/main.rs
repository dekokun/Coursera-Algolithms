#![feature(test)]
#![feature(rand)]
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::fmt;

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();
    stdin.read_line(input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("please input positive number");
    let mut uf = UF::new(n);

    for line in stdin.lock().lines() {
        let tmp = line.unwrap();
        let split = tmp.split(" ");
        // println!("{}", line.unwrap());
        let vec = split.collect::<Vec<&str>>();
        let p: usize = vec[0].parse().expect("please input positive number");
        let q: usize = vec[1].parse().expect("please input positive number");
        if !uf.connected(p, q) {
            uf.union(p, q);
            println!("{} {}", p, q);
        }

    }
}

pub struct UF {
    id: HashMap<usize, usize>,
}

impl UF {
    fn new(_: usize) -> UF {
        UF { id: HashMap::new() }
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        if p == q {
            return true;
        }
        let pid = self.id.get(&p);
        let qid = self.id.get(&q);
        if pid == None || qid == None {
            false
        } else {
            pid == qid
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        // borrowingの回避用中間変数
        // もっとうまくできないものか
        let mut is_p_none = false;
        let pid;
        {
            let pid_result = self.id.get(&p);
            pid = match pid_result {
                Some(x) => *x,
                None => {
                    is_p_none = true;
                    p
                }
            };
        }
        if is_p_none {
            self.id.insert(p, p);
        }

        let qid;
        let mut is_q_none = false;
        {
            let qid_result = self.id.get(&q);
            qid = match qid_result {
                Some(x) => *x,
                None => {
                    is_q_none = true;
                    q
                }
            };
        }
        if is_q_none {
            self.id.insert(q, q);
        }
        let mut id = self.id.clone();
        for (key, value) in id.iter_mut() {
            if *value == pid {
                *self.id.get_mut(key).unwrap() = qid;
            }
        }
    }
}

pub struct QuickUnionUF {
    id: Vec<usize>,
}

impl QuickUnionUF {
    fn new(n: usize) -> QuickUnionUF {
        let mut vec: Vec<usize> = Vec::new();
        for i in 0..n {
            vec.push(i);
        }
        QuickUnionUF { id: vec }
    }

    fn root(&self, p: usize) -> usize {
        let mut i = p;
        while self.id[i] != i {
            i = self.id[i];
        }
        i
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn union(&mut self, p: usize, q: usize) {
        let proot = self.root(p);
        let qroot = self.root(q);
        self.id[proot] = qroot
    }
}

pub struct QuickUnionWeightedUF {
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl QuickUnionWeightedUF {
    fn new(n: usize) -> QuickUnionWeightedUF {
        let mut vec: Vec<usize> = Vec::new();
        let mut vec2: Vec<usize> = Vec::new();
        for i in 0..n {
            vec.push(i);
            vec2.push(1);
        }
        QuickUnionWeightedUF {
            id: vec,
            sz: vec2,
        }
    }

    fn root(&self, p: usize) -> usize {
        let mut i = p;
        while self.id[i] != i {
            i = self.id[i];
        }
        i
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn union(&mut self, p: usize, q: usize) {
        let proot = self.root(p);
        let qroot = self.root(q);
        if proot == qroot {
            return;
        }
        if self.sz[proot] < self.sz[qroot] {
            self.id[proot] = qroot;
            self.sz[qroot] += self.sz[proot];
        } else {
            self.id[qroot] = proot;
            self.sz[proot] += self.sz[qroot];
        }
    }
}

impl fmt::Display for QuickUnionWeightedUF {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self.id)
    }
}

pub struct QuickUnionWeightedFlattenUF {
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl QuickUnionWeightedFlattenUF {
    fn new(n: usize) -> QuickUnionWeightedFlattenUF {
        let mut vec: Vec<usize> = Vec::new();
        let mut vec2: Vec<usize> = Vec::new();
        for i in 0..n {
            vec.push(i);
            vec2.push(1);
        }
        QuickUnionWeightedFlattenUF {
            id: vec,
            sz: vec2,
        }
    }

    fn root(&mut self, p: usize) -> usize {
        let mut i = p;
        while self.id[i] != i {
            self.id[i] = self.id[self.id[i]];
            i = self.id[i];
        }
        i
    }

    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn union(&mut self, p: usize, q: usize) {
        let proot = self.root(p);
        let qroot = self.root(q);
        if proot == qroot {
            return;
        }
        if self.sz[proot] < self.sz[qroot] {
            self.id[proot] = qroot;
            self.sz[qroot] += self.sz[proot];
        } else {
            self.id[qroot] = proot;
            self.sz[proot] += self.sz[qroot];
        }
    }
}

#[cfg(test)]
pub struct UFSimple {
    id: Vec<usize>,
}

#[cfg(test)]
impl UFSimple {
    fn new(n: usize) -> UFSimple {
        let mut vec: Vec<usize> = Vec::new();
        for i in 0..n {
            vec.push(i);
        }
        UFSimple { id: vec }
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.id[p as usize] == self.id[q as usize]
    }
    fn union(&mut self, p: usize, q: usize) {
        let pid = self.id[p];
        let qid = self.id[q];
        for i in 0..self.id.len() {
            if self.id[i] == pid {
                self.id[i] = qid;
            }
        }
    }
}

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate test;
extern crate rand;
mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use std::cmp;
    #[cfg(test)]
    use test::Bencher;
    #[cfg(test)]
    use rand::Rng;
    #[test]
    fn self_always_connected_self() {
        let uf = UF::new(1);
        debug_assert_eq!(uf.connected(0, 0), true);
    }

    #[test]
    fn not_connected() {
        let uf = UF::new(2);
        debug_assert_eq!(uf.connected(0, 1), false);
    }

    #[test]
    fn normal() {
        let mut uf = UF::new(2);
        uf.union(0, 1);
        debug_assert_eq!(uf.connected(0, 1), true);
    }

    #[test]
    fn multi() {
        let mut uf = UF::new(5);
        uf.union(0, 1);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        uf.union(3, 4);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        debug_assert_eq!(uf.connected(0, 3), false);
        uf.union(0, 4);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        debug_assert_eq!(uf.connected(0, 3), true);
        debug_assert_eq!(uf.connected(0, 4), true);
    }

    #[test]
    fn quf_not_connected() {
        let uf = QuickUnionUF::new(2);
        debug_assert_eq!(uf.connected(0, 1), false);
    }

    #[test]
    fn quf_normal() {
        let mut uf = QuickUnionUF::new(2);
        uf.union(0, 1);
        debug_assert_eq!(uf.connected(0, 1), true);
    }

    #[test]
    fn quf_multi() {
        let mut uf = QuickUnionUF::new(5);
        uf.union(0, 1);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        uf.union(3, 4);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        debug_assert_eq!(uf.connected(0, 3), false);
        uf.union(0, 4);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        debug_assert_eq!(uf.connected(0, 3), true);
        debug_assert_eq!(uf.connected(0, 4), true);
    }
    #[test]
    fn qwuf_not_connected() {
        let uf = QuickUnionWeightedUF::new(2);
        debug_assert_eq!(uf.connected(0, 1), false);
    }

    #[test]
    fn qwuf_normal() {
        let mut uf = QuickUnionWeightedUF::new(2);
        uf.union(0, 1);
        debug_assert_eq!(uf.connected(0, 1), true);
    }

    #[test]
    fn qwuf_multi() {
        let mut uf = QuickUnionWeightedUF::new(5);
        uf.union(0, 1);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        uf.union(3, 4);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        debug_assert_eq!(uf.connected(0, 3), false);
        uf.union(0, 4);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        debug_assert_eq!(uf.connected(0, 3), true);
        debug_assert_eq!(uf.connected(0, 4), true);
    }

    #[test]
    fn qwfuf_not_connected() {
        let mut uf = QuickUnionWeightedFlattenUF::new(2);
        debug_assert_eq!(uf.connected(0, 1), false);
    }

    #[test]
    fn qwfuf_normal() {
        let mut uf = QuickUnionWeightedFlattenUF::new(2);
        uf.union(0, 1);
        debug_assert_eq!(uf.connected(0, 1), true);
    }

    #[test]
    fn qwfuf_multi() {
        let mut uf = QuickUnionWeightedFlattenUF::new(5);
        uf.union(0, 1);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        uf.union(3, 4);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        debug_assert_eq!(uf.connected(0, 3), false);
        uf.union(0, 4);
        debug_assert_eq!(uf.connected(0, 1), true);
        debug_assert_eq!(uf.connected(0, 2), false);
        debug_assert_eq!(uf.connected(2, 3), false);
        debug_assert_eq!(uf.connected(0, 3), true);
        debug_assert_eq!(uf.connected(0, 4), true);
    }

    #[test]
    quickcheck! {
        #[ignore]
        fn uf_is_same_uf2(p1: usize, q1: usize, p2: usize, q2: usize) -> bool {
            let max1 = cmp::max(p1, q1);
            let max2 = cmp::max(p2, q2);
            let max = cmp::max(max1, max2);
            let mut uf = UF::new(max + 1);
            let mut uf2 = UFSimple::new(max + 1);
            uf.union(p2, q2);
            uf2.union(p2, q2);
            uf.connected(p1, q1) == uf2.connected(p1, q1)
        }
    }
    quickcheck! {
        #[ignore]
        fn quickunion_is_same_uf(p1: usize, q1: usize, p2: usize, q2: usize) -> bool {
            let max1 = cmp::max(p1, q1);
            let max2 = cmp::max(p2, q2);
            let max = cmp::max(max1, max2);
            let mut uf = UFSimple::new(max + 1);
            let mut quickunion = QuickUnionUF::new(max + 1);
            uf.union(p2, q2);
            quickunion.union(p2, q2);
            let uf = uf.connected(p1, q1);
            let quf = quickunion.connected(p1, q1);
            uf == quf
        }
    }
    quickcheck! {
        #[ignore]
        fn quickunionweited_is_same_uf(p1: usize, q1: usize, p2: usize, q2: usize) -> bool {
            let max1 = cmp::max(p1, q1);
            let max2 = cmp::max(p2, q2);
            let max = cmp::max(max1, max2);
            let mut uf = UF::new(max + 1);
            let mut uf2 = QuickUnionWeightedUF::new(max + 1);
            uf.union(p2, q2);
            uf2.union(p2, q2);
            uf.connected(p1, q1) == uf2.connected(p1, q1)
        }
    }
    #[bench]
    fn bench_uf(b: &mut Bencher) {
        b.iter(|| {
            let mut uf = UF::new(1000);
            uf.union(10, 20);
            uf.connected(5, 10)
        });
    }
    #[bench]
    fn bench_uf2(b: &mut Bencher) {
        b.iter(|| {
            let mut uf = UFSimple::new(1000);
            uf.union(10, 20);
            uf.connected(5, 10)
        });
    }
    #[bench]
    fn bench_quf(b: &mut Bencher) {
        b.iter(|| {
            let mut uf = QuickUnionUF::new(1000);
            uf.union(10, 20);
            uf.connected(5, 10)
        });
    }

    #[bench]
    fn bench_qwuf(b: &mut Bencher) {
        b.iter(|| {
            let mut uf = QuickUnionWeightedUF::new(1000);
            uf.union(10, 20);
            uf.connected(5, 10)
        });
    }

    #[bench]
    fn bench_many_connect_uf(b: &mut Bencher) {
        let max = 1000;
        let mut uf = UF::new(max);
        let count = 1000;
        let mut rng = rand::IsaacRng::new_unseeded();
        b.iter(|| {
            for _ in 0..count {
                let p = rng.gen_range(0, max - 1);
                let q = rng.gen_range(0, max - 1);
                uf.union(p, q);
            }
        });
    }
    #[bench]
    fn bench_many_connect_quf(b: &mut Bencher) {
        let max = 1000;
        let mut uf = QuickUnionUF::new(max);
        let count = 1000;
        let mut rng = rand::IsaacRng::new_unseeded();
        b.iter(|| {
            for _ in 0..count {
                let p = rng.gen_range(0, max - 1);
                let q = rng.gen_range(0, max - 1);
                uf.union(p, q);
            }
        });
    }

    #[bench]
    fn bench_many_connect_qwuf(b: &mut Bencher) {
        b.iter(|| {
            let max = 1000;
            let mut uf = QuickUnionWeightedUF::new(max);
            let count = 1000;
            let mut rng = rand::IsaacRng::new_unseeded();
            for _ in 0..count {
                let p = rng.gen_range(0, max - 1);
                let q = rng.gen_range(0, max - 1);
                uf.union(p, q);
            }
        });
    }

    #[bench]
    fn bench_many_connect_qwfuf(b: &mut Bencher) {
        b.iter(|| {
            let max = 1000;
            let mut uf = QuickUnionWeightedFlattenUF::new(max);
            let count = 1000;
            let mut rng = rand::IsaacRng::new_unseeded();
            for _ in 0..count {
                let p = rng.gen_range(0, max - 1);
                let q = rng.gen_range(0, max - 1);
                uf.union(p, q);
            }
        });
    }

    #[bench]
    fn bench_many_find_uf(b: &mut Bencher) {
        let max = 1000;
        let mut uf = UF::new(max);
        let union_count = 1000;
        let find_count = 1000;
        let mut rng = rand::IsaacRng::new_unseeded();
        for _ in 0..union_count {
            let p = rng.gen_range(0, max - 1);
            let q = rng.gen_range(0, max - 1);
            uf.union(p, q);
        }
        b.iter(|| {
            for _ in 0..find_count {
                let p = rng.gen_range(0, max - 1);
                let q = rng.gen_range(0, max - 1);
                uf.connected(p, q);
            }
        });
    }
    #[bench]
    fn bench_many_find_quf(b: &mut Bencher) {
        let max = 1000;
        let mut uf = QuickUnionUF::new(max);
        let union_count = 1000;
        let find_count = 1000;
        let mut rng = rand::IsaacRng::new_unseeded();
        for _ in 0..union_count {
            let p = rng.gen_range(0, max - 1);
            let q = rng.gen_range(0, max - 1);
            uf.union(p, q);
        }
        b.iter(|| {
            for _ in 0..find_count {
                let p = rng.gen_range(0, max - 1);
                let q = rng.gen_range(0, max - 1);
                uf.connected(p, q);
            }
        });
    }

    #[bench]
    fn bench_many_find_qwuf(b: &mut Bencher) {
        let max = 1000;
        let mut uf = QuickUnionWeightedUF::new(max);
        let union_count = 1000;
        let find_count = 1000;
        let mut rng = rand::IsaacRng::new_unseeded();
        for _ in 0..union_count {
            let p = rng.gen_range(0, max - 1);
            let q = rng.gen_range(0, max - 1);
            uf.union(p, q);
        }
        b.iter(|| {
            for _ in 0..find_count {
                let p = rng.gen_range(0, max - 1);
                let q = rng.gen_range(0, max - 1);
                uf.connected(p, q);
            }
        });
    }

    #[bench]
    fn bench_many_find_qwfuf(b: &mut Bencher) {
        let max = 1000;
        let mut uf = QuickUnionWeightedFlattenUF::new(max);
        let union_count = 1000;
        let find_count = 1000;
        let mut rng = rand::IsaacRng::new_unseeded();
        for _ in 0..union_count {
            let p = rng.gen_range(0, max - 1);
            let q = rng.gen_range(0, max - 1);
            uf.union(p, q);
        }
        b.iter(|| {
            for _ in 0..find_count {
                let p = rng.gen_range(0, max - 1);
                let q = rng.gen_range(0, max - 1);
                uf.connected(p, q);
            }
        });
    }
}
