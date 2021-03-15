# Rusty Time

A simple timer.

### Example

```toml
# Add this to your [dependencies] section in Cargo.toml
rusty_timer = "0.11.3"
```

```rust
// main.rs
use rusty_timer::Timer;

fn main() {
    // Create a timer ahead of time
    let mut timer = Timer::from_millis(500);
    // In your game- or event-loop, updated the timer
    loop {
        let delta = Duration::from_millis(16);
        timer.update(delta);
        if timer.ready {
            break;
        }
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

If you like `rusty_time`, please consider [sponsoring me] on GitHub. 💖
