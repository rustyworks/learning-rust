fn main() {
    unless!(false, {
        println!("Inside unless!")
    });

    let result = benchmark!({2 ^ 100});
    println!("benchmark result: {result}");
    let result = eval!(2 ^ 100);
    println!("eval result: {result}");
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
