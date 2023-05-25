fn increment_string(s: &str) -> String {
    if s.is_empty() {
        "1".to_string()
    } else {
        s.chars().rfold(
            (String::new(), true),
            |(result, carry), next| {
                let (addition_to_result, new_carry): (String, bool) =
                    match next.to_digit(10) {
                        None => {
                            if carry {
                                (format!("1{next}"), false)
                            } else {
                                (next.to_string(), false)
                            }
                        },
                        Some(digit) => {
                            if carry {
                                if digit == 9 {
                                    ("0".to_string(), true)
                                } else {
                                    ((digit + 1).to_string(), false)
                                }
                            } else {
                                (digit.to_string(), false)
                            }
                        }
                    };
                (result + &addition_to_result, new_carry)
            }
        ).0.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected,
                "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
    }
}