pub trait RequiredFields {
    fn required_fields() -> &'static [&'static str] {
        &[]
    }
}
