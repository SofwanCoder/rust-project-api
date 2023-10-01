pub trait Service {
    fn init() -> Self;
    fn get_name(&self) -> &str;
    fn get_service_scope(&mut self);
}
