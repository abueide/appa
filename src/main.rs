use std::env;

fn greet(name: Option<&str>) -> String {
    match name {
        Some(name) => format!("Hello, {}! Welcome to appa CLI tool.", name),
        None => "Hello, world! Welcome to appa CLI tool.".to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let name = &args[1];
        println!("{}", greet(Some(name)));
    } else {
        println!("{}", greet(None));
        println!("Usage: appa [name]");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_with_name() {
        let result = greet(Some("Alice"));
        assert_eq!(result, "Hello, Alice! Welcome to appa CLI tool.");
    }

    #[test]
    fn test_greet_without_name() {
        let result = greet(None);
        assert_eq!(result, "Hello, world! Welcome to appa CLI tool.");
    }

    #[test]
    fn test_greet_empty_string() {
        let result = greet(Some(""));
        assert_eq!(result, "Hello, ! Welcome to appa CLI tool.");
    }
}
