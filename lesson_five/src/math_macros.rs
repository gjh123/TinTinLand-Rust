#[macro_export]
macro_rules! calculate {
    ($x:expr, $operator:tt, $y:expr) => {
        match $operator {
            "+" => $x + $y,
            "-" => $x - $y,
            "*" => $x * $y,
            "/" => $x / $y,
            _ => panic!("Unsupported operator: {}", $operator),
        }
    };
}
