use std::collections::HashMap;

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        return "".to_string()
    }

    let mut book_categories_and_counts: HashMap<char, u32> = HashMap::new();

    list_art
        .iter()
        .for_each( |book_code| {
            let category = book_code.chars().collect::<Vec<_>>()[0];
            let count_str = book_code.split(" ").collect::<Vec<_>>()[1];
            let count = count_str.parse::<u32>().unwrap();
            *book_categories_and_counts.entry(category).or_insert(0) += count;
        });

    list_cat
        .iter().map(|category| {
            let category_char = category.chars().collect::<Vec<_>>()[0];
            let count = book_categories_and_counts.get(&category_char).unwrap_or(&0);
            format!("({category_char} : {count})")
        })
        .collect::<Vec<_>>()
        .join(" - ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");

    }

    #[test]
    fn empty_book_list() {
        let list_art = vec![];
        let list_cat = vec!["B", "R", "D", "X"];

        dotest(list_art, list_cat, "");
    }
}
