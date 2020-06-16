use snowcat_derive::{Blueprint, Transferable};
use snowcat::core::connect::Connect;
use snowcat::core::conditions::{Condition, Operator};
use snowcat::core::traits::{Blueprint, Transferable};

mod utils;

#[derive(Blueprint, Transferable)]
struct Article {
    title: String,
    text: String,
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
