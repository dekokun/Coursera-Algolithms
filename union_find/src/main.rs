use std::io;
use std::io::BufRead;

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

#[cfg(test)]
mod test {
    use super::*;
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
}
