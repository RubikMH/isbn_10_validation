fn valid_isbn10(isbn: &str) -> bool {
    let mut result = false;
    let mut sum = 0;

    if isbn.len() == 10 {
        for (index, item) in isbn.chars().enumerate() {
            if index == 9 && item == 'X' {
                sum = sum + ((index + 1) * 10)
            }
            if item.is_numeric() {
                let num = item.to_string().parse::<usize>().unwrap();
                sum = sum + ((index + 1) * num)
            } else {
                break;
            }
        }
    }
    if sum != 0 && sum % 11 == 0 {
        result = true
    }

    result
}

fn dotest(isbn: &str, expected: bool) {
    let actual = valid_isbn10(isbn);
    assert!(
        actual == expected,
        "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}"
    )
}

fn sample_tests() {
    dotest("1112223339", true);
    dotest("048665088X", true);
    dotest("1293000000", true);
    dotest("1234554321", true);
    dotest("1234512345", false);
    dotest("1293", false);
    dotest("X123456788", false);
    dotest("ABCDEFGHIJ", false);
    dotest("XXXXXXXXXX", false);
    dotest("123456789T", false);
}

fn main() {
    sample_tests();
}
