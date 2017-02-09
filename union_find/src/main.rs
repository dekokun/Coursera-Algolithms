#![feature(test)]
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

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
        let pid = self.id.get(&p);
        let qid = self.id.get(&q);
        if pid == None || qid == None {
          false
        } else {
          pid == qid
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        let pid = self.id.get(&p);
        let pid = match pid {
          Some(x) => *x,
          None => {
            self.id.insert(p, p);
            p
          }
        };
        let qid = self.id.get(&q);
        let qid = match qid {
          Some(x) => *x,
          None => {
            self.id.insert(q, q);
            q
          }
        };
    }
}

pub struct UF2 {
    id: Vec<usize>,
}

impl UF2 {
    fn new(n: usize) -> UF2 {
        let mut vec: Vec<usize> = Vec::new();
        for i in 0..n {
            vec.push(i);
        }
        UF2 { id: vec }
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
mod tests {
    use super::*;
    use std::cmp;
    use test::Bencher;
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

    quickcheck! {
        #[ignore]
        fn uf_is_same_uf2(p1: usize, q1: usize, p2: usize, q2: usize) -> bool {
            let max1 = cmp::max(p1, q1);
            let max2 = cmp::max(p2, q2);
            let max = cmp::max(max1, max2);
            let mut uf = UF::new(max + 1);
            let mut uf2 = UF2::new(max + 1);
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
            let mut uf = UF2::new(1000);
            uf.union(10, 20);
            uf.connected(5, 10)
        });
    }
}
