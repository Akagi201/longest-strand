pub fn lcs(x: &[u8], y: &[u8]) -> Vec<u8> {
    // Initialize the LCS table
    let m = x.len();
    let n = y.len();
    let mut l = vec![vec![0; n + 1]; m + 1];

    // Fill in the LCS table
    for i in 0..=m {
        for j in 0..=n {
            if i == 0 || j == 0 {
                l[i][j] = 0;
            } else if x[i - 1] == y[j - 1] {
                l[i][j] = l[i - 1][j - 1] + 1;
            } else {
                l[i][j] = std::cmp::max(l[i - 1][j], l[i][j - 1]);
            }
        }
    }

    // Backtrack to find the LCS
    let mut i = m;
    let mut j = n;
    let mut index = l[i][j];
    let mut lcs = Vec::with_capacity(index as usize);
    while i > 0 && j > 0 {
        if x[i - 1] == y[j - 1] {
            lcs.push(x[i - 1]);
            i -= 1;
            j -= 1;
            index -= 1;
        } else if l[i - 1][j] > l[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    lcs.reverse();
    lcs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs() {
        let x = b"abcdefg";
        let y = b"abcefg";
        let expected = b"abcefg";
        let result = lcs(x, y);
        assert_eq!(result, expected);

        let x = b"abcdefg";
        let y = b"defg";
        let expected = b"defg";
        let result = lcs(x, y);
        assert_eq!(result, expected);

        let x = b"abcdefg";
        let y = b"xyz";
        let expected = b"";
        let result = lcs(x, y);
        assert_eq!(result, expected);

        let x = b"abcdefg";
        let y = b"abcfeg";
        let expected = b"abcfg";
        let result = lcs(x, y);
        assert_eq!(result, expected);
    }
}
