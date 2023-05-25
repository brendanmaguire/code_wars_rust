fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        None
    } else {
        let total_seconds_to_catch = 3600 * g / (v2 - v1);

        let hours = total_seconds_to_catch / 3600;
        let minutes = (total_seconds_to_catch / 60)  % 60;
        let seconds = total_seconds_to_catch % 60;

        Some(vec![hours, minutes, seconds])
    }
}

#[test]
fn basic_tests() {
    assert_eq!(race(820, 81, 550), None);
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(479, 604, 65), Some(vec![0, 31, 12]));
}