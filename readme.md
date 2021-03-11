# ansi-colors-macro

> A proc-macro for creating colored ANSI strings at compile-time.

## Usage

```rust
use ansi_colors_macro::ansi_string;

const DONE_MSG: &str = ansi_string!("{green Done}");
```

## Shortcomings

- Supports foreground colors only. The syntax has to be expanded for background colors and styling as well. PRs welcome. Please open an issue so we can exchange ideas beforehand!

#### License

<sup>
Licensed under either of <a href="license-apache">Apache License, Version
2.0</a> or <a href="license-mit">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
