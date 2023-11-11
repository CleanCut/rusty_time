# Rusty Time

A simple timer. See also the [reference docs on docs.rs](https://docs.rs/rusty_time/latest/rusty_time/)

## Quick Start

Add `rusty_timer` to your `Cargo.toml` with:

```shell
cargo add rusty_timer
```

Then use it like this:

```rust
fn main() {
    let mut timer = Timer::new(Duration::from_secs_f32(1.5));

    let mut start_time = Instant::now();
    loop {
        timer.tick(start_time.elapsed());
        start_time = Instant::now();
        println!(
            "Time on timer: {:.2}s ({:.1}%)",
            timer.remaining().as_secs_f32(),
            timer.percent_left() * 100.0
        );
        if timer.just_finished() {
            break;
        }
    }
    println!("Timer finished!");
}
```

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

## Sponsor

If you like `rusty_time`, please consider [sponsoring me](https://github.com/sponsors/CleanCut) on GitHub. 💖
