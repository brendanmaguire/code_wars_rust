use std::cmp::{max, min};

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        None
    } else {
        let mut lng = lng;
        let mut wdth = wdth;

        let mut squares = Vec::new();

        while lng != wdth {
            let current_square_len = min(lng, wdth);
            squares.push(current_square_len);
            wdth = (lng - wdth).abs();
            lng = current_square_len;
        }

        squares.push(lng);
        Some(squares)
    }
}

fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {

    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);

    testing(6, 4, Some(vec![4, 2, 2]));
    testing(6, 2, Some(vec![2, 2, 2]));

}
