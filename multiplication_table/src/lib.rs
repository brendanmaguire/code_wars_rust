fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    for i in 1..=len {
        result.push(
            (1..=len).map(|j| i * j).collect::<Vec<_>>()
        );
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1,2,3], [2,4,6], [3,6,9]]);
        assert!(multiplication_table(0).is_empty());
        assert_eq!(multiplication_table(4), [[1,2,3,4], [2,4,6,8], [3,6,9,12], [4,8,12,16]]);
    }
}