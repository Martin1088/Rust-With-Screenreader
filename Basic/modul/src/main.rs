
mod greet {
    pub fn hello() {
        println!("Hello");
    }

    pub fn goodmorning() {
        println!("Good Mornichg");
    }
}

fn all_caps(word: &str) -> String {
    word.to_uppercase()
}
fn main() {
    greet::hello();
    greet::goodmorning();

}
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "string is uppercase");
    }
}
