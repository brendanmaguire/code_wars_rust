fn revrot(s: &str, c: usize) -> String {
    if c == 0 {
        return String::new();
    }

    let result_len = s.len() - (s.len() % c);
    let digits =
        s.chars()
            .take(result_len)
            .collect::<Vec<_>>();

    let mut result = String::new();

    for chunk in digits.chunks(c) {
        let sum_of_cubes = chunk.iter()
            .map(|digit| {
                let digit_int = digit.to_digit(10).expect("Non digit char found");
                digit_int * digit_int * digit_int
            })
            .sum::<u32>();

        let mut result_chars = chunk.iter().collect::<Vec<_>>();

        if sum_of_cubes % 2 == 0 {
            result_chars.reverse();
        } else {
            result_chars.rotate_left(1);
        }

        result.push_str(&result_chars.iter().map(|c| c.to_string()).collect::<String>());
    }

    result
}

fn testing(s: &str, c: usize, exp: &str) -> () {
    assert_eq!(&revrot(s, c), exp)
}

#[test]
fn basics_revrot() {
    testing("1234", 0, "");
    testing("", 0, "");
    testing("1234", 5, "");

    let s = "73304";
    testing(s, 5, "33047");

    let s = "733049910872815764";
    testing(s, 5, "330479108928157");

    let s = "73304991087281576455176044327690580265896";
    // 73304991 08728157 64551760 44327690 58026589 6
    testing(s, 8, "1994033775182780067155464327690480265895");
}