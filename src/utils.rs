#[macro_export]
macro_rules! c {
    ($x:tt = $y:tt) => {
        Condition {
            field_name: stringify!($x).to_string(),
            operator: Operator::Eq,
            value: stringify!($y).to_string(),
        };
    }
}