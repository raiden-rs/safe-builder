use safe_builder::*;

#[derive(Builder, Debug)]
#[builder(method_name = "renamed_builder")]
pub struct User {
    id: String,
    name: String,
    addr: Option<String>,
}

fn main() {
    let hello = User::renamed_builder()
        .id("hello".to_owned())
        .name("bokuweb".to_string())
        .build();
    dbg!(hello);
}
