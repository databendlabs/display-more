# display-more

A Rust utility crate providing enhanced display formatting for various types.

## Features

- **Display Option**: Format `Option<T>` values with customizable display
- **Display Result**: Format `Result<T, E>` values
- **Display Slice**: Format slices with configurable element limits
- **Display Unix Epoch**: Convert Unix timestamps to human-readable datetime strings

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
display-more = "0.2.0"
```

## Examples

### Display Option

```rust
use display_more::DisplayOptionExt;

let value: Option<i32> = Some(42);
println!("{}", value.display());  // "42"

let none: Option<i32> = None;
println!("{}", none.display());   // "None"
```

### Display Result

```rust
use display_more::DisplayResultExt;

let ok = Result::<i32, &str>::Ok(42);
println!("{}", ok.display());  // "Ok(42)"

let err = Result::<i32, &str>::Err("error");
println!("{}", err.display());  // "Err(error)"
```

### Display Slice

```rust
use display_more::DisplaySliceExt;

let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
println!("{}", numbers.display());     // "[1,2,3,4,..,8]"
println!("{}", numbers.display_n(3));  // "[1,2,..,8]"
```

### Display Unix Epoch

```rust
use std::time::Duration;
use display_more::DisplayUnixTimeStampExt;

let timestamp = Duration::from_millis(1723102819023);
println!("{}", timestamp.display_unix_timestamp());        // "2024-08-08T07:40:19.023000Z+0000"
println!("{}", timestamp.display_unix_timestamp_short());  // "2024-08-08T07:40:19.023"
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
