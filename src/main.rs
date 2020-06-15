use snowcat::{Blueprint, Transferable, Condition, Operator, Connect};
use snowcat_derive::{Blueprint, Transferable};

#[derive(Blueprint, Transferable)]
struct Article {
    title: String,
    text: String,
}

macro_rules! c {
    ($x:tt = $y:tt) => {
        Condition {
            field_name: stringify!($x).to_string(),
            operator: Operator::Eq,
            value: stringify!($y).to_string(),
        };
    }
}

fn main() {
    Connect::<Article>::new("127.0.0.1:9999")
        .insert(Article {
            title: "New article".to_string(),
            text: "Some text".to_string(),
        })
        .execute();

    Connect::<Article>::new("127.0.0.1:9999")
        .filter(c!(title = 12))
        .execute();
}
