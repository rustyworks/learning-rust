// extern crate proc_macro;

// use macros_tinkering::simple_macro;
// use proc_macro::TokenStream;

fn main() {
    unless!(false, {
        println!("Inside unless!")
    });

    unless_else!(false, {
        println!("Unless, condition false")
    } else {
        println!("Unless, condition true")
    });

    let result = benchmark!({2 ^ 100});
    println!("benchmark result: {result}");
    let result = eval!(2 ^ 100);
    println!("eval result: {result}");

    // my_function();
}

#[macro_export]  // This are needed to ensure macro load first before used.
macro_rules! unless {
    ($e:expr, $code:block) => {
        if !$e {
            $code
        };
    };
}

#[macro_export]
macro_rules! unless_else {
    ($e:expr, $code:block else $else_code:block) => {
        if !$e {
            $code
        } else {
            $else_code
        }
    };
}

#[macro_export]
macro_rules! benchmark {
    ($code:block) => {{ // we can return value of the macro using double parentheses: {{ }}
        let start_time = std::time::Instant::now();
        let result = { $code };
        let duration = start_time.elapsed();
        println!("The calling of this method is: {:?}", duration);
        result
    }}
}

#[macro_export]
macro_rules! eval {
    ($e:expr) => {{
        $e
    }}
}

// #[timer]
// fn do_something(_n: TokenStream) {
//     println!("a")
// }

// #[simple_macro]
// fn my_function() {
//     println!("This is my function.");
// }
