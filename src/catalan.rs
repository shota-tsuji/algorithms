fn catalan(n: u64) -> u64 {
    // Base case
    if n <= 1 {
        return 1;
    }

    // catalan(n) is sum of catalan(i)*catalan(n-i-1)
    let mut res = 0;
    for i in 0..n {
        res += catalan(i) * catalan(n - i - 1);
    }

    res
}

// A dynamic programming based function to find nth Catalan number
fn catalan_dp(n: u64) -> u64 {
    let mut catalan = vec![0; (n + 2) as usize];
    catalan[0] = 1;
    catalan[1] = 1;

    for i in 2..=n as usize {
        for j in 0..i {
            catalan[i] += catalan[j] * catalan[i - j - 1];
        }
    }

    catalan[n as usize]
}

fn main() {
    for i in 0..10 {
        print!("{} ", catalan(i));
    }
    println!();

    for i in 0..10 {
        print!("{} ", catalan_dp(i));
    }
}