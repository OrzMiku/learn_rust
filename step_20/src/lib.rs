// Declarative macro
// use marco_rules to define a macro
// use marco_export to export the macro
// macro_rules! macro_name {
//      (Rule1) => {
//          (Code)
//      };
//      (Rule2) => {
//          (Code)
//      };
//      ...
//      (RuleN) => {
//          (Code)
//      };
// }

#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello, world!");
    };
    // $[identifier] : [fragment-specifier]
    // $name:expr means expression
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

#[macro_export]
macro_rules! map {
    // ty means type
    ($key:ty, $value:ty) => {
        std::collections::HashMap::<$key, $value>::new()
    };

    // ($(pattern), *) means zero or more repetitions of the pattern
    ($($key:expr => $value:expr), *) => {
        {
            let mut map = std::collections::HashMap::new();
            // $()* will expand to the contents of the macro
            $(
                map.insert($key, $value);
            )*
            map
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hello() {
        hello!();
        hello!("Rust");
    }

    #[test]
    fn test_map() {
        let _map = map!(String, Vec<String>);
        let teams = map! {
            "Team A" => vec!["Alice".to_string(), "Bob".to_string()],
            "Team B" => vec!["Charlie".to_string(), "David".to_string()],
            "Team C" => vec!["Eve".to_string(), "Frank".to_string()]
        };

        for (team, members) in teams.iter() {
            println!("{}: {:?}", team, members);
        }
    }
}