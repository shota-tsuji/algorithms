fn print_power_set(set: Vec<char>) {
    // the size of power set of a set (i.e. 2^n)
    let power_set_size = 2u64.pow(set.len() as u32);

    // Run from counter 000..0 to 111..1
    for i in 0..power_set_size {
        for j in 0..set.len() {
            // Check if jth bit in the counter is set
            // If set then print jth element from set
            if (i & (1 << j)) > 0 {
                print!("{}", set[j]);
            }
        }
        println!();
    }
}

fn main() {
    let set = vec!['a', 'b', 'c'];
    print_power_set(set);
}