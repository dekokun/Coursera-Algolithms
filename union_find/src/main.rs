use std::io;
use std::io::prelude::*;

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

struct UF {
    id: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> UF {
        let mut vec: Vec<usize> = Vec::new();
        for i in 0..n {
            vec.push(i);
        }
        UF { id: vec }
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

