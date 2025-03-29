macro_rules! run_block {
    ($b:block) => {
        {
            let mut _block = $b;
            _block
        }
    };
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };

    ($a:expr, $($rest:expr),+) => {
        $a + add!($($rest),+)
    };
}

macro_rules! define_function {
    ($name: ident) => {
        fn $name() {
            println!("Function {} called", stringify!($name));
        }
    };
}

#[allow(unused_macros)]
macro_rules! deplicate_item {
    ($i: item) => {
        $i $i
    };
}

macro_rules! print_lifetime {
    ($lt:lifetime) => {
        println!("lifetime: {}", stringify!($lt));
    };
}

macro_rules! print_literal {
    ($lit:literal) => {
        println!("literal: {}", stringify!($lit));
    };
}

macro_rules! print_meta {
    ($m:meta) => {
        println!("meta: {}", stringify!($m));
    };
}

macro_rules! match_pat {
    ($value: expr, $pattern:pat => $body:expr) => {
        match $value {
            $pattern => {
                $body
            }
            _ => {
                println!("No match");
            }
        }
    };
}

macro_rules! use_crate {
    ($p: path) => {
        use $p;
    };
}

macro_rules! run_statement {
    ($s: stmt) => {
        $s
    };
}

macro_rules! print_tt {
    ($($t: tt)+) => {
        println!("Token tree: {}", stringify!($($t)+));
    };
}

macro_rules! print_ty {
    ($t: ty) => {
        println!("Type: {}", stringify!($t));
    };
}

macro_rules! print_vis {
    ($v: vis) => {
        println!("Visibility: {}", stringify!($v));
    };
}

fn main() {
    use_crate!(step_20::hello);
    hello!();
    hello!("Rust");

    use_crate!(step_20::map);
    let map = map!(String, Vec<String>);
    let map2 = map! {
        "key1" => vec!["value1".to_string(), "value2".to_string()],
        "key2" => vec!["value3".to_string(), "value4".to_string()],
        "key3" => vec!["value5".to_string(), "value6".to_string()]
    };
    println!("Map: {:?}", map);
    println!("Map2: {:?}", map2);

    run_block!({
        let x = 5;
        println!("x = {}", x);
    });

    let sum = add!(1, 2, 3, 4, 5);
    println!("Sum = {}", sum);

    define_function!(my_function);
    my_function();

    // The following code will cause a compilation error because the function is defined twice
    // deplicate_item! {
    //     fn duplicated_function() {
    //         println!("This function is duplicated");
    //     }
    // }

    print_lifetime!('a);
    print_literal!("Hello, world!");
    print_meta!(derive(Debug));
    
    let value = Some(10);
    let none_valie: Option<i32> = None;
    match_pat!(value, Some(x) => {
        println!("Matched: {}", x);
    });
    match_pat!(none_valie, Some(x) => {
        println!("Matched: {}", x);
    });

    use_crate!(std::collections::HashMap);
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("Map: {:?}", map);

    run_statement!(let x = 10);
    run_statement!(println!("x = {}", x));

    print_tt!(1 + 2 * 3 / 4);
    print_tt!(if true { 1 } else { 0 });
    print_ty!(i32);
    print_vis!(pub(crate));
}