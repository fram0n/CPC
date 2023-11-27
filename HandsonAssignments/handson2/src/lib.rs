use std::cmp;

pub struct SegmentTreeMax {
    pub size: usize,
    pub tree: Vec<u64>,
    pub lazy: Vec<u64>,
}

pub struct SegmentTreeSeg {
    pub size: usize,
    pub tree: Vec<i64>,
}

impl SegmentTreeMax {

    // initializes the segment tree with length 4*n, all values set to 0
    pub fn new_tree(n: usize) -> Self {
        Self {
            size: n,
            tree: vec![0; n*4],
            lazy: vec![(n as u64)*2; n*4],
        }
    }

    /// builds the segment tree storing the maximum value
    pub fn build(&mut self, a: &Vec<u64>, v: usize, tl: usize, tr: usize) {
        if tl == tr {
            self.tree[v] = a[tl-1];
        }
        else {
            let tm = (tl + tr - 1) / 2;
            self.build(a, v*2, tl, tm);
            self.build(a, v*2+1, tm+1, tr);
            self.tree[v] = cmp::max(self.tree[v*2], self.tree[v*2+1]);
        }
    }

    /// returns the maximum of the given range
    pub fn max(&mut self, v: usize, tl: usize, tr: usize, i: usize, j: usize) -> u64 {
        if i > j {
            return 0;
        }
        if i == tl && j == tr {
            return self.tree[v];
        }
        self.push(v);
        let tm: usize = (tl + tr - 1) / 2;
        return cmp::max(self.max(v*2, tl, tm, i, cmp::min(j, tm)), self.max(v*2+1, tm+1, tr, cmp::max(i,tm+1), j));
    }

    // updates the segment tree using the lazy propagation approach
    pub fn update(&mut self, v: usize, tl: usize, tr: usize, i: usize, j: usize, val: u64) {
        if i > j { 
            return;
        }
        if i == tl && j == tr {
            let updt = cmp::min(self.tree[v], val);
            if self.tree[v] != updt {
                self.tree[v] = updt;
                self.lazy[v] = val;
            }
        } else {
            self.push(v);
            let tm = (tl + tr - 1) / 2;
            self.update(v*2, tl, tm, i, cmp::min(j, tm), val);
            self.update(v*2+1, tm+1, tr, cmp::max(i, tm+1), j, val);
            self.tree[v] = cmp::max(self.tree[v*2], self.tree[v*2+1]);
        }
    }

    // propagates lazy updates from a parent node to its children in the segment tree
    pub fn push(&mut self, v: usize) {
        if self.lazy[v] != (self.size as u64)*2 {
            let val = self.lazy[v];
            if val < self.tree[v*2] {
                self.tree[v*2] = val;
                self.lazy[v*2] = val;
            }
            if val < self.tree[v*2+1] {
                self.tree[v*2+1] = val;
                self.lazy[v*2+1] = val;
            }
            self.lazy[v] = (self.size as u64)*2;
        }
    }
}

impl SegmentTreeSeg {

    pub fn new_tree(n: usize) -> Self {
        Self {
            size: n,
            tree: vec![0; n*4],
        }
    }

    // builds the segment tree
    pub fn build(&mut self, a: &Vec<i64>, v: usize, tl: usize, tr: usize) {
        if tl == tr {
            self.tree[v] = a[tl];
        }
        else {
            let tm = (tl + tr) / 2;
            self.build(a, v*2, tl, tm);
            self.build(a, v*2+1, tm+1, tr);
            self.tree[v] = cmp::max(self.tree[v*2], self.tree[v*2+1]);
        }
    }

    // checks if there exists a position within the range [i,j] where the value is equal to a given value k
    pub fn is_there(&mut self, v: usize, tl: usize, tr: usize, i: usize, j: usize, k:i64) -> u64 {
        if i > j {
            return 0;
        }
        if i == tl && j == tr {
            if self.tree[v] == k {
                return 1;
            }
            else if tl != tr {
                let tm: usize = (tl + tr) / 2;
                return cmp::max(self.is_there(v*2, tl, tm, i, cmp::min(j, tm), k), self.is_there(v*2+1, tm+1, tr, cmp::max(i,tm+1), j, k));
            }
            else {
                return 0;
            }
        }
        let tm: usize = (tl + tr) / 2;
        return cmp::max(self.is_there(v*2, tl, tm, i, cmp::min(j, tm), k), self.is_there(v*2+1, tm+1, tr, cmp::max(i,tm+1), j, k));
    }
}