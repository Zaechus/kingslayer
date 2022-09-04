pub trait Entity {
    fn name(&self) -> &str;

    fn desc(&self) -> &str;

    fn inspect(&self) -> &str;
}
