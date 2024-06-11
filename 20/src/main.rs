fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input");
    let needle = input.trim().parse::<usize>().unwrap();
    let s = eratosthenes(1000000);
    let i = s.into_iter().position(|n| { n >= (needle / 10) });
    println!("Answer #1 is {}", i.unwrap());
    let s = eratosthenes2(1000000);
    let i = s.into_iter().position(|n| { n >= needle });
    println!("Answer #2 is {}", i.unwrap());
}

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut r = Vec::new();
    r.resize(n, 1);
    let limit = n;
    for i in 2..limit {
        for j in (i..limit).step_by(i) {
            r[j] += i;
        }
    }
    r
}

fn eratosthenes2(n: usize) -> Vec<usize> {
    let mut r = Vec::new();
    r.resize(n, 11);
    let limit = n;
    for i in 2..limit {
        let mut count = 0;
        for j in (i..limit).step_by(i) {
            r[j] += i * 11;
            count += 1;
            if count == 50 { break; }
        }
    }
    r
}
