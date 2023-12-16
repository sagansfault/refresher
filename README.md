# refresher
A simple value interval-refresher.

Each time the value is accessed, it is checked to see if it has expired its refresh-duration and updated if so.
```rust
let mut value = AccessRefresher::new_fn(get_time, Duration::from_secs(1)).await;
println!("{:?}", value.get().await); // time
tokio::time::sleep(Duration::from_secs(2)).await;
println!("{:?}", value.get().await); // time + 2 seconds

async fn get_time() -> Duration {
    Instant::now()
}
```