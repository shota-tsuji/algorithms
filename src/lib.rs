struct BWT {
    // Length of original text
    size: u64,
    // Index of first character of the original text
    head: usize,
    // Original text
    str: Option<String>,
    // Suffix Array
    sa: Vec<u64>,
}

impl BWT {
    fn new() -> Self {
        BWT {
            size: 0,
            head: 0,
            str: None,
            sa: Vec::new(),
        }
    }

    fn sa2char(&self, i: usize, depth: u64) -> u8 {
        let offset = ((self.sa[i] + depth) % self.size) as usize;
        // not to lowercase
        self.str.as_ref().unwrap().chars().nth(offset).unwrap().to_ascii_lowercase() as u8
    }

    fn sort(&mut self, begin: i64, end: i64, depth: u64) {
        let mut a = begin;
        let mut b = begin;
        let mut c = end;
        let mut d = end;
        let size = end - begin + 1;
        if size <= 1 {
            return;
        }

        //let pivot_pos = rand::random::<usize>() * size;
        let pivot_pos = (rand::random::<f64>() * size as f64) as i64;
        let pivot = self.sa2char((begin + pivot_pos) as usize, depth);

        while b <= c {
            let mut b_ch = self.sa2char(b as usize, depth);
            while b <= c && b_ch <= pivot {
                if b_ch == pivot {
                    self.sa.swap(a as usize, b as usize);
                    a += 1;
                }
                b += 1;
                if b >= self.size as i64 { break; }
                b_ch = self.sa2char(b as usize, depth);
            }

            let mut c_ch = self.sa2char(c as usize, depth);
            while b <= c && c_ch >= pivot {
                if c_ch == pivot {
                    self.sa.swap(c as usize, d as usize);
                    d -= 1;
                }
                c -= 1;
                if c < 0 { break; }
                c_ch = self.sa2char(c as usize, depth);
            }

            if b > c { break; }

            self.sa.swap(b as usize, c as usize);
            b += 1;
            c -= 1;
        }

        let eq_size_a = (a - begin).min(b - a);
        for i in 0..eq_size_a {
            self.sa.swap((begin + i) as usize, (b - eq_size_a + i) as usize);
        }

        let eq_size_d = (d - c).min(end - d);
        for i in 0..eq_size_d {
            self.sa.swap((b + i) as usize, (end - eq_size_d + i + 1) as usize);
        }

        self.sort(begin, begin + (b - a) - 1, depth);
        self.sort(begin + (b - a), end - (d - c), depth + 1);
        self.sort(end - (d - c) + 1, end, depth);
        self.print_sa();
    }

    fn build(&mut self, str: &str) {
        let str = str.clone().to_owned() + "\0";
        self.str = Some(str.to_owned());
        self.sa.clear();
        self.size = 0;

        for c in self.str.clone().unwrap().chars() {
            print!(", {}", c);
        }

        for _ in str.chars() {
            self.sa.push(self.size);
            self.size += 1;
        }
        /*
        while str.chars().nth(self.size as usize).unwrap() != '\0' {
            self.sa.push(self.size);
            self.size += 1;
        }
         */
        self.print_sa();

        self.sort(0, (self.size - 1) as i64, 0);

        for i in 0..self.size as usize {
            if self.sa[i] == 0 {
                self.head = i;
                break;
            }
        }
    }

    fn get(&self, i: usize) -> char {
        if i >= self.size as usize {
            panic!("bwt::get()");
        }
        let index = self.sa[i];
        self.str.as_ref().unwrap().chars().nth(((index + self.size - 1) % self.size) as usize).unwrap()
    }

    fn get_str(&self) -> String {
        let mut result = String::new();
        for i in 0..self.size as usize {
            result.push(self.get(i))
        }
        result
    }

    fn print_sa(&self) {
        print!("sa[");
        for i in 0..self.sa.len() {
            print!("{}, ", self.sa[i]);
        }
        println!("]");

        print!("[");
        for i in 0..self.sa.len() {
            print!("{}, ", self.get(i));
        }
        println!("]");
    }
}

#[cfg(test)]
mod tests {
    use crate::BWT;

    #[test]
    fn build() {
        let mut bwt = BWT::new();
        bwt.build("mississippi");
        println!("{}", bwt.get_str());
        assert_eq!(bwt.get_str(), "ipssm\0pissii".to_string())
    }
}