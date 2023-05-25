fn solution(s: &str) -> String {
    s
        .chars().rev().collect::<String>()
        .split_inclusive(char::is_uppercase)
        .map(|reversed_word| reversed_word.chars().rev().collect::<String>())
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
