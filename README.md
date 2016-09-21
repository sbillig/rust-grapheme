# Note: This library has not been maintained in a while. You should probably be using the [unicode-segmentation](https://crates.io/crates/unicode-segmentation) crate instead.

rust-grapheme
=============

Unicode grapheme cluster segmentation (UAX #29) for strings in Rust.

Installation:

    rust pkg install github.com/sbillig/rust-grapheme.git

Use:

```rust
extern mod grapheme;
use grapheme::GraphemeList;

fn main() {
    let s = "u\u0308\u00fc";

    let by = s.len();
    let ch = s.char_len();
    let cl = s.graphemes().len();

    println!("bytes: {}, chars: {}, clusters: {}", by, ch, cl);

    for c in s.graphemes() {
        // c is a str slice
        println!("{}", c);
    }
}
```
Output:
```
bytes: 5, chars: 3, clusters: 2
ü
ü
```
