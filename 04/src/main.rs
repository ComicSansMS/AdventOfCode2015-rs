use md5;

fn main() {
    let input = "yzbqklnj";
    let answer1 = try_hashes(input);
    println!("Answer #1 is {}", answer1);
    let answer2 = try_hashes2(input);
    println!("Answer #2 is {}", answer2);
}

fn try_hashes(input: &str) ->i32 {
    try_hashes_impl(input, check_hash)
}

fn try_hashes2(input: &str) ->i32 {
    try_hashes_impl(input, check_hash2)
}

fn try_hashes_impl(input: &str, check_func: fn(data: &str) -> bool) -> i32 {
    let mut count = 0;
    loop {
        if check_func(&format!("{}{}", input, count)) {
            break;
        }
        count += 1;
    }
    count
}

fn check_hash(data: &str) -> bool {
    let digest = md5::compute(data);
    digest.iter().take(2).all(|x| *x == 0) && *digest.iter().nth(2).unwrap() < 16
}

fn check_hash2(data: &str) -> bool {
    let digest = md5::compute(data);
    digest.iter().take(3).all(|x| *x == 0)
}

#[cfg(test)]
mod tests {
    use crate::check_hash;
    use crate::try_hashes;

    #[test]
    fn test_md5() {
        assert_eq!(format!("{:?}", md5::compute("abcdef609043")), "000001dbbfa3a5c83a2d506429c7b00e"); 
        assert_eq!(format!("{:?}", md5::compute("pqrstuv1048970")), "000006136ef2ff3b291c85725f17325c"); 
    }
    #[test]
    fn test_check_hash() {
        assert!(!check_hash("abcdef"));
        assert!(check_hash("abcdef609043"));
        assert!(!check_hash("pqrstuv"));
        assert!(check_hash("pqrstuv1048970"));
    }

    #[test]
    fn test_try_hashes() {
        assert_eq!(try_hashes("abcdef"), 609043);
        assert_eq!(try_hashes("pqrstuv"), 1048970);
    }
}
