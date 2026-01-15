pub trait Report {
    fn get_status_report(&self) -> impl std::future::Future<Output = String>;
}

impl<T: Report> Report for &T {
    async fn get_status_report(&self) -> String {
        T::get_status_report(self).await
    }
}

pub struct Reporter<T> {
    inner: T,
}

impl Reporter<Identity> {
    pub fn new() -> Reporter<Identity> {
        Reporter { inner: Identity }
    }
}

impl Default for Reporter<Identity> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Reporter<T> {
    pub fn add_item<U: Report>(self, item: U) -> Reporter<Both<T, U>> {
        Reporter {
            inner: Both::new(self.inner, item),
        }
    }
}

impl<T: Report> Report for Reporter<T> {
    async fn get_status_report(&self) -> String {
        self.inner.get_status_report().await
    }
}

pub struct Identity;

impl Report for Identity {
    async fn get_status_report(&self) -> String {
        "".to_string()
    }
}

pub struct Both<R1, R2> {
    inner1: R1,
    inner2: R2,
}

impl<R1, R2> Both<R1, R2> {
    fn new(inner1: R1, inner2: R2) -> Self {
        Both { inner1, inner2 }
    }
}

impl<R1: Report, R2: Report> Report for Both<R1, R2> {
    async fn get_status_report(&self) -> String {
        let part1 = self.inner1.get_status_report().await;
        let part2 = self.inner2.get_status_report().await;

        format!("{}\n{}", part1, part2)
    }
}
