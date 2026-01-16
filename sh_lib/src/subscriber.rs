pub trait Subscribe {
    fn on_event(&mut self, name: String);
}

impl<F> Subscribe for F
where
    F: FnMut(String),
{
    fn on_event(&mut self, name: String) {
        self(name);
    }
}
