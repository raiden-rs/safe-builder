#[cfg(test)]
mod tests {

    #[cfg(test)]
    use safe_builder::*;

    #[derive(Builder, PartialEq, Debug)]
    pub struct User {
        id: String,
        name: String,
        addr: Option<String>,
    }
    #[test]
    fn test_user() {
        let me = User::builder()
            .id("1234".to_owned())
            .name("bokuweb".to_owned())
            .build();
        assert_eq!(
            User {
                id: "1234".to_owned(),
                name: "bokuweb".to_owned(),
                addr: None,
            },
            me
        )
    }
}
