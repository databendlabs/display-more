# display-more

A Rust utility crate that provides enhanced display formatting for various types.

## Overview

`display-more` is a helper library that extends the standard `Display` trait functionality in Rust, providing more flexible and customizable display formatting for common types such as:

- `Option<T>` values
- Slices
- Unix epoch timestamps

## Features

- **Display Option**: Enhanced formatting for `Option<T>` values
- **Display Slice**: Customizable display formatting for slice types
- **Display Unix Epoch**: Human-readable formatting for Unix timestamp values

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
display-more = "0.1.0"
```

## Examples

### Display Option

```rust
use display_more::display_option::DisplayOption;

let value: Option<i32> = Some(42);
println!("Value: {}", value.display());
```

### Display Slice

```rust
use display_more::display_slice::DisplaySlice;

let numbers = [1, 2, 3, 4, 5];
println!("Numbers: {}", numbers.display());
```

### Display Unix Epoch

```rust
use display_more::display_unix_epoch::DisplayUnixEpoch;

let timestamp = 1671234567;
println!("Time: {}", timestamp.display());
```

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
