use std::{usize, mem::swap};

#[derive(Default, Debug)]
pub struct Dsu {
    n: i32,
    parent_or_size: Vec::<i32>
}

impl Dsu {
    pub fn new(n: i32) -> Self {
        Dsu {
            n,
            parent_or_size: vec![-1, n]
        }
    }

    pub fn leader(&mut self, a: i32) -> i32 {
        let a_usize: usize = a as usize;
        if self.parent_or_size[a_usize] < 0 {
            return a;
        } 

        self.parent_or_size[a_usize] = self.leader(a);
        
        self.parent_or_size[a_usize]
    }

    pub fn merge(&mut self, a: i32, b: i32) -> i32 {
        let mut x = self.leader(a) as usize;
        let mut y = self.leader(b) as usize;

        if x == y {
            return x as i32;
        }

        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            swap(&mut x, &mut y);
        }

        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;

        x as i32
    }

    pub fn size(&mut self, a: i32) -> i32 {
        let leader = self.leader(a) as usize;
        self.parent_or_size[leader]
    }

    pub fn is_same(&mut self, a: i32, b: i32) -> bool {
        let a = self.leader(a);
        let b = self.leader(b);

        a == b
    }

    pub fn groups(&mut self) -> Vec<Vec<i32>> {
        let mut leader_buf: Vec<i32> = vec![0, self.n];
        let mut groups_size: Vec<i32> = vec![0, self.n];

        for i in 0..self.n {
            let leader = self.leader(i);
            leader_buf[i as usize] = leader;
            groups_size[leader_buf[i as usize] as usize] += 1;
        } 

        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..self.n {
            result.push(Vec::with_capacity(groups_size[i as usize] as usize));
        }

        for i in 0..self.n {
            result[leader_buf[i as usize] as usize].push(i);
        }

        result.retain(|x| !x.is_empty());

        result
    }
}
