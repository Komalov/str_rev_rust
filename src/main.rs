fn reverse_str(string: &str) -> String {
    let mut str_as_vec: Vec<char> = string.chars().collect();
    for i in 0..str_as_vec.len() / 2 {
        let last_i = str_as_vec.len() - 1 - i;
        let temp = str_as_vec[i];
        str_as_vec[i] = str_as_vec[last_i];
        str_as_vec[last_i] = temp;
    }

    str_as_vec.into_iter().collect()
}

fn main() {
    let result = reverse_str("hello");
    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::reverse_str;

    #[test]
    fn basic_happy_path_test() {
        let modified = "olleh";
        let original = "hello";
        let result = reverse_str(original);
        assert_eq!(modified, result.to_owned());
    }
}
