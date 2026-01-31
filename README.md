Simple, single-threaded runtime for running asynchronous code in Rust using async/await primitives

Usage:

- Clone this repository
- Insert it's local path in the dependencies of Cargo.toml file
- Write an async code

Example:

```rust
// Example available in ./examples/timer_future.rs

// TimeFuture implementation ...

fn main() {
    // Creates the runtime
    let rt = async_runtime::build();

    // Spawn two async blocks
    rt.spawn(async {
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    rt.spawn(async {
        println!("howdy! 2");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done! 2");
    });

    // Since it's single-threaded, it's necessary an explicity call to the runtime entry point
    rt.start();
}
```