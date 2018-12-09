use std::io::stdin;

type Integer = i32;

// FNVハッシュはキーのビット数が小さい時はSipHashよりも高速。
use fnv::FnvHashMap;

fn primes() -> Box<Iterator<Item = Integer>> {
    let mut multiples = FnvHashMap::with_hasher(Default::default());;
    let iter = (3..).step_by(2).filter_map(move |i| {
        let (prime_or_none, factor) = match multiples.remove(&i) {
            Some(f) => (None, f),
            None => (Some(i), i * 2),
        };

        (1..)
            .map(|j| i + j * factor)
            .skip_while(|m| multiples.contains_key(m))
            .next()
            .map(|m| multiples.insert(m, factor));

        prime_or_none // filter_map()は次の素数が見つかるまでリトライする。
    });
    // 最初に2を返してから、他の素数を返す。
    Box::new((2..).take(1).chain(iter))
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
    for p in primes() {
        if is_4949(p) {
            ans.push(p);
            if ans.len() >= n {
                break;
            }
        }
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
fn test_primes() {
    let ans = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    for (p1, p2) in primes().zip(ans) {
        assert_eq!(p1, p2);
    }
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
