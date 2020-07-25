use safe_builder::*;

#[derive(Builder, Debug)]
pub struct User {
    id: String,
    addr: Option<String>,
}

fn main() {
    let hello = User::builder()
        .id("hello".to_owned())
        .build();
    dbg!(hello);
}
