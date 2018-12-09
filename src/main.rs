use std::io::stdin;

type Integer = i32;

struct PrimeSeries {
    primes: Vec<Integer>,
}

impl PrimeSeries {
    fn new() -> PrimeSeries {
        PrimeSeries {
            primes: vec![2, 3, 5],
        }
    }

    fn expand(&mut self, max2: Integer) {
        for i in (self.primes.last().unwrap() + 2..max2).step_by(2) {
            if i * i > max2 {
                break;
            }
            if self.is_prime(i) {
                self.primes.push(i);
            }
        }
    }

    /// 現在の自前の素数列だけから素数判定を行う
    /// 適切に ``expand`` しておかないとバグる
    /// 特にこだわりがないなら ``determine_prime`` を使うべき
    fn is_prime(&self, n: Integer) -> bool {
        if n <= 1 {
            return false;
        }
        for p in &self.primes {
            if p * p > n {
                return true;
            }
            if n % p == 0 {
                return false;
            }
        }
        true
    }

    fn determine_prime(&mut self, n: Integer) -> bool {
        self.expand(n);
        self.is_prime(n)
    }
}

fn is_4949(mut n: Integer) -> bool {
    loop {
        if n == 0 {
            return false;
        }
        match n % 10 {
            4 | 9 => return true,
            _ => n /= 10,
        }
    }
}

fn solve_4949(n: usize) -> Vec<Integer> {
    let mut ans = Vec::new();
    let mut ps = PrimeSeries::new();
    let mut i = 1;
    while ans.len() < n {
        if ps.determine_prime(i) && is_4949(i) {
            ans.push(i);
        }
        i += 1;
    }
    ans
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse().expect("fail parse as integer");
    let ans = solve_4949(n);
    println!(
        "{}",
        ans.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(","),
    );
}

#[test]
fn test_prime() {
    let mut ps = PrimeSeries::new();
    assert_eq!(ps.determine_prime(0), false);
    assert_eq!(ps.determine_prime(1), false);
    assert_eq!(ps.determine_prime(2), true);
    assert_eq!(ps.determine_prime(9), false);
    assert_eq!(ps.determine_prime(57), false);
    assert_eq!(ps.determine_prime(97), true);
    assert_eq!(ps.determine_prime(9999991), true);
}

#[test]
fn test_4949() {
    assert_eq!(is_4949(0), false);
    assert_eq!(is_4949(1), false);
    assert_eq!(is_4949(4), true);
    assert_eq!(is_4949(9), true);
    assert_eq!(is_4949(87318112), false);
    assert_eq!(is_4949(987318112), true);
}
