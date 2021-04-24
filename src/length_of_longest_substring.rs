use std::collections::HashSet;

pub fn length_of_longest_substring(s: &str) -> usize {
    
    let mut left = 0;
    let mut len: usize = 0;
    let mut max_len: usize = 0;
    let mut win: HashSet<char> = HashSet::new();

    let chars: Vec<char> = s.chars().collect();
    

    for c in &chars {
        len += 1;
        while win.contains(c) {
            win.remove(&chars[left]);
            len -= 1;
            left += 1;
        }
        win.insert(*c);
        if len > max_len {
            max_len = len;
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn abc_len_3() {
        const S: &str = "abc";
        const S_EXPECTED_LEN: usize = 3;
        let len = length_of_longest_substring(S);
        assert_eq!(len, S_EXPECTED_LEN);
    }

    #[test]
    fn aaa_len_1() {
        const S: &str = "aaa";
        const S_EXPECTED_LEN: usize = 1;
        let len = length_of_longest_substring(S);
        assert_eq!(len, S_EXPECTED_LEN);
    }

    #[test]
    fn pwwkew_len_3() {
        const S: &str = "pwwkew";
        const S_EXPECTED_LEN: usize = 3;
        let len = length_of_longest_substring(S);
        assert_eq!(len, S_EXPECTED_LEN);
    }
}