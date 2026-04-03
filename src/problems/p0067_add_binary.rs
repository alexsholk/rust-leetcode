//! https://leetcode.com/problems/add-binary/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a.bytes().rev();
        let mut b = b.bytes().rev();
        let mut carry = 0;
        let mut res = Vec::new();

        while a.len() > 0 || b.len() > 0 || carry > 0 {
            let sum = a.next().unwrap_or(b'0') - b'0'
                + b.next().unwrap_or(b'0') - b'0'
                + carry;

            res.push(b'0' + sum % 2);
            carry = sum / 2;
        }

        res.reverse();
        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!("100", Solution::add_binary("11".to_string(), "1".to_string()));
        assert_eq!("10101", Solution::add_binary("1010".to_string(), "1011".to_string()));
        assert_eq!("0", Solution::add_binary("0".to_string(), "0".to_string()));
    }
}
