# Rusty Time

A simple timer.

### Example

Either run `cargo add rusty_timer` or manually add the following to the `[dependencies]` section of your `Cargo.toml` file.

```toml
rusty_timer = "0.11.3"
```

```rust
// main.rs
use rusty_time::Timer;

fn main() {
    // Create a timer
    let mut timer = Timer::from_millis(500);

    // In some sort of game-loop or event-loop, update the timer
    loop {
        let delta = std::time::Duration::from_millis(16);
        timer.update(delta);
        if timer.ready {
            println!("Ready!");
            break;
        }
        println!("Time left: {:?}", timer.time_left);
    }
}
```

### Historical Note

`rusty_time` was part of [`rusty_engine`] up until after version `0.11.1`.

[`rusty_engine`]: https://github.com/cleancut/rusty_engine

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

## Sponsor

If you like `rusty_time`, please consider [sponsoring me](https://github.com/sponsors/CleanCut) on GitHub. 💖
