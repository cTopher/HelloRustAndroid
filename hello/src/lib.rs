pub fn hello(name: &str) -> String {
    format!("Hello, {name}! Greetings from Ferris.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello("Christopher");
        assert_eq!(result, "Hello, Christopher! Greetings from Ferris.");
    }
}
