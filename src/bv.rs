const SMALL_BLOCK_SIZE: u64 = 64;
const BLOCK_RATE: u64 = 8;
const LARGE_BLOCK_SIZE: u64 = 512;

fn rank64(x: u64, i: u64, b: bool) -> u64 {
    let mut x = x;
    if !b {
        x = !x;
    }
    x <<= SMALL_BLOCK_SIZE - i;
    // Count bits in u64 and sum them in-place.
    x = ((x & 0xaaaaaaaaaaaaaaaa) >> 1) + (x & 0x5555555555555555);
    x = ((x & 0xcccccccccccccccc) >> 2) + (x & 0x3333333333333333);
    x = ((x & 0xf0f0f0f0f0f0f0f0) >> 4) + (x & 0x0f0f0f0f0f0f0f0f);
    x = ((x & 0xff00ff00ff00ff00) >> 8) + (x & 0x00ff00ff00ff00ff);
    x = ((x & 0xffff0000ffff0000) >> 16) + (x & 0x0000ffff0000ffff);
    x = ((x & 0xffffffff00000000) >> 32) + (x & 0x00000000ffffffff);
    x
}

fn select64(x: u64, mut i: u64, b: bool) -> u64 {
    let mut x = x;
    if !b {
        x = !x;
    }
    let x1 = ((x & 0xaaaaaaaaaaaaaaaa) >> 1) + (x & 0x5555555555555555);
    let x2 = ((x1 & 0xcccccccccccccccc) >> 2) + (x1 & 0x3333333333333333);
    let x3 = ((x2 & 0xf0f0f0f0f0f0f0f0) >> 4) + (x2 & 0x0f0f0f0f0f0f0f0f);
    let x4 = ((x3 & 0xff00ff00ff00ff00) >> 8) + (x3 & 0x00ff00ff00ff00ff);
    let x5 = ((x4 & 0xffff0000ffff0000) >> 16) + (x4 & 0x0000ffff0000ffff);

    // Binary search `i`th bit in u64
    i += 1;
    let mut pos = 0;
    let v5 = x5 & 0xffffffff;
    if i > v5 {
        i -= v5;
        pos += 32;
    }
    let v4 = (x4 >> pos) & 0x0000ffff;
    if i > v4 {
        i -= v4;
        pos += 16;
    }
    let v3 = (x3 >> pos) & 0x000000ff;
    if i > v3 {
        i -= v3;
        pos += 8;
    }
    let v2 = (x2 >> pos) & 0x0000000f;
    if i > v2 {
        i -= v2;
        pos += 4;
    }
    let v1 = (x1 >> pos) & 0x00000003;
    if i > v1 {
        i -= v1;
        pos += 2;
    }
    let v0 = (x >> pos) & 0x00000001;
    if i > v0 {
        i -= v0;
        pos += 1;
    }
    pos
}

struct BitVector {
    // vector of the element which is 64-bits vector
    v: Vec<u64>,
    // cumulative sum (rank1) per 8-bits on v_
    r: Vec<u64>,
    size: u64,
    size1: u64,
}

impl BitVector {
    fn new() -> Self {
        BitVector {
            v: Vec::new(),
            r: Vec::new(),
            size: 0,
            size1: 0,
        }
    }

    fn set(&mut self, i: u64, b: bool) {
        if i >= self.size { self.size = i + 1; }
        let q = i / SMALL_BLOCK_SIZE;
        let r = i % SMALL_BLOCK_SIZE;
        while q >= self.v.len() as u64 {
            self.v.push(0);
        }
        let m = 0x1 << r;
        if b { self.v[q as usize] |= m; } else { self.v[q as usize] &= !m; }
    }

    fn size(&self, b: bool) -> u64 {
        if b { self.size1 } else { self.size - self.size1 }
    }

    fn build(&mut self) {
        self.r.clear();
        self.size1 = 0;

        for i in 0..self.v.len() {
            if i % BLOCK_RATE as usize == 0 {
                self.r.push(self.size(true));
            }
            self.size1 += rank64(self.v[i], SMALL_BLOCK_SIZE, true);
        }
    }

    fn rank(&self, i: u64, b: bool) -> u64 {
        if i > self.size { panic!("BitVector::rank()"); }
        if i == 0 { return 0; }

        let i = i - 1;
        let q_large = i / LARGE_BLOCK_SIZE;
        let q_small = i / SMALL_BLOCK_SIZE;
        let r = i % SMALL_BLOCK_SIZE;
        let mut rank = self.r[q_large as usize];
        if !b {
            rank = q_large * LARGE_BLOCK_SIZE - rank;
        }
        let begin = q_large * BLOCK_RATE;
        for j in begin..q_small {
            rank += rank64(self.v[j as usize], SMALL_BLOCK_SIZE, b);
        }
        rank += rank64(self.v[q_small as usize], r + 1, b);
        rank
    }

    /// Returns the position which is set `i`th bit.
    ///
    /// # Arguments
    ///
    /// * `i` - the number of bits counted from LSB
    /// * `b` - true for set(1) bit or false for clear(0) one
    fn select(&self, i: u64, b: bool) -> u64 {
        if i > self.size { panic!("BitVector::select()"); }

        let mut i = i;
        let mut left = 0;
        let mut right = self.r.len() as u64;
        while left < right {
            let pivot = (left + right) / 2;
            let mut rank = self.r[pivot as usize];
            if !b { rank = pivot * LARGE_BLOCK_SIZE - rank; }
            if i < rank { right = pivot; } else { left = pivot + 1; }
        }
        right -= 1;

        if b { i -= self.r[right as usize]; } else { i -= right * LARGE_BLOCK_SIZE - self.r[right as usize]; }
        // sum of the number of (8 x 64bits) blocks (512bits)
        let mut j = right * BLOCK_RATE;
        loop {
            let rank = rank64(self.v[j as usize], SMALL_BLOCK_SIZE, b);
            if i < rank { break; }
            j += 1;
            i -= rank;
        }
        j * SMALL_BLOCK_SIZE + select64(self.v[j as usize], i, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank64_() {
        assert_eq!(rank64(12, 4, true), 2);
        assert_eq!(rank64(12, 3, true), 1);
    }

    #[test]
    fn select64_() {
        assert_eq!(select64(12, 1, true), 3);
        assert_eq!(select64(12, 0, true), 2);
        assert_eq!(select64(12, 2, false), 4);
    }

    #[test]
    fn rank_successfully() {
        let mut bv = BitVector::new();
        bv.build();
        assert_eq!(bv.rank(0, true), 0);

        let mut bv = BitVector::new();
        bv.set(0, true);
        bv.set(3, true);
        bv.set(8, true);
        bv.build();
        assert_eq!(bv.rank(9, true), 3);
    }

    #[test]
    fn select_successfully() {
        let mut bv = BitVector::new();
        bv.set(0, true);
        bv.set(3, true);
        bv.set(8, true);
        bv.build();
        assert_eq!(bv.select(0, true), 0);
        assert_eq!(bv.select(2, true), 8);
    }
}