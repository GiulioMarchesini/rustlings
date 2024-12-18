fn trim_me(input: &str) -> &str {
    input.trim()
}

fn compose_me(input: &str) -> String {
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // input.replace("cars", "balloons") // too easy
    let to_find: &str = "cars";
    let to_replace: &str = "balloons";

    match input.find(to_find) {
        Some(index) => {
            let mut new_string = String::new();
            new_string.push_str(&input[..index]);
            new_string.push_str(to_replace);
            new_string.push_str(&input[index + to_find.len()..]);
            new_string
        }
        None => input.to_string(),
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
