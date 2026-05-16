// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    let plus_solution = input.to_string() + " world!";
    let format_solution = format!("{} world!", input);
    let to_owned_solution = input.to_owned() + " world!";
    let mut push_str_solution = input.to_string();
    push_str_solution.push_str(" world!");

    return push_str_solution;
}

fn replace_me(input: &str) -> String {
    let output: String = input.replace("cars", "balloons");
    output
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
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars, lot of cars"),
            "I love to look at balloons, lot of balloons"
        );
    }
}
