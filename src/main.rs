use std::io::stdin;

type Integer = i32;

fn is_prime(n: Integer) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
        if i * i > n {
            return true;
        }
    }
    true
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
    let mut i = 1;
    while ans.len() < n {
        if is_prime(i) && is_4949(i) {
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
    assert_eq!(is_prime(0), false);
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(57), false);
    assert_eq!(is_prime(97), true);
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
