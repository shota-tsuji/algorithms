const SMALL_BLOCK_SIZE: u64 = 64;
fn rank64(x: u64, i: u64, b: bool) -> u64 {
    let mut x = x;
    if !b {
        x = !x;
    }
    x <<= (SMALL_BLOCK_SIZE - i);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank() {
        assert_eq!(rank64(12, 4, true), 2);
        assert_eq!(rank64(12, 3, true), 1);
    }

    #[test]
    fn select() {
        assert_eq!(select64(12, 1, true), 3);
        assert_eq!(select64(12, 0, true), 2);
        assert_eq!(select64(12, 2, false), 4);
    }
}