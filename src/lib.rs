use std::future::Future;
use std::time::Duration;

use tokio::time::Instant;

pub struct AccessRefresher<T, Fut: Future<Output=T>, F: Fn() -> Fut> {
    value: T,
    accessed: Instant,
    refresh: Duration,
    load_fn: F
}

impl<T, Fut: Future<Output=T>, F: Fn() -> Fut> AccessRefresher<T, Fut, F> {
    pub fn new(value: T, load_fn: F, refresh: Duration) -> Self {
        let accessed = Instant::now();
        Self { value, accessed, refresh, load_fn, }
    }

    pub async fn new_fn(load_fn: F, refresh: Duration) -> Self {
        let initial_value = load_fn().await;
        Self::new(initial_value, load_fn, refresh)
    }

    pub async fn get(&mut self) -> &T {
        let now = Instant::now();
        if self.accessed.elapsed() > self.refresh {
            self.value = (self.load_fn)().await;
            self.accessed = now;
        }
        &self.value
    }
}

#[tokio::test]
async fn test() {
    let func = || async {
        Instant::now()
    };
    let mut value = AccessRefresher::new_fn(func, Duration::from_secs(1)).await;
    println!("{:?}", value.get().await);
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("{:?}", value.get().await);
}
