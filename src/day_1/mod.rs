pub fn solve(input: String) -> String {
    input.lines().map(str_to_int).sum::<i32>().to_string()
}

fn str_to_int(input: &str) -> i32 {
    let s = input.trim();

    if s.is_empty() {
        return 0;
    }
    let i: i32 = match s.parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positives_are_converted() {
        assert_eq!(str_to_int("+1"), 1);
    }

    #[test]
    fn negatives_are_converted() {
        assert_eq!(str_to_int("-1"), -1);
    }

    #[test]
    fn empty_lines_return_0() {
        assert_eq!(str_to_int(""), 0);
    }

    #[test]
    fn positives_sums() {
        assert_eq!(solve("+1\n+1\n+1".to_string()), "3");
    }

    #[test]
    fn negatives_sums() {
        assert_eq!(solve("-1\n-2\n-3".to_string()), "-6");
    }

    #[test]
    fn mixed_sums() {
        assert_eq!(solve("+1\n+1\n-2".to_string()), "0");
    }
}
