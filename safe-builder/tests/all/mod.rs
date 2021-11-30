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
    fn smoke_test() {
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

    #[derive(Builder, PartialEq, Debug)]
    #[builder(method_name = "foo_builder")]
    pub struct Foo {
        bar: String,
    }
    #[test]
    fn rename_builder_test() {
        let foo = Foo::foo_builder().bar("bokuweb".to_owned()).build();
        assert_eq!(
            Foo {
                bar: "bokuweb".to_owned(),
            },
            foo
        )
    }

    #[derive(Builder, PartialEq, Debug)]
    pub struct RawIdentifier {
        id: String,
        r#type: String,
    }
    #[test]
    fn raw_identifier_test() {
        let raw_ident = RawIdentifier::builder()
            .id("1234".to_owned())
            .r#type("type".to_owned())
            .build();
        assert_eq!(
            RawIdentifier {
                id: "1234".to_owned(),
                r#type: "type".to_owned(),
            },
            raw_ident
        )
    }
}
