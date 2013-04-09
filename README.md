rust-grapheme
=============

Unicode grapheme cluster segmentation (UAX #29) for strings in Rust.

Installation:

    rust pkg install github.com/sbillig/rust-grapheme.git

Use:

```rust
extern mod grapheme;

fn main() {
    let s = "u\u0308\u00fc";

    let by = s.len(),
        ch = s.char_len(),
        cl = grapheme::cluster_count(s);

    io::println(fmt!("bytes: %?, chars: %?, clusters: %?", by, ch, cl));

    for grapheme::each_cluster(s) |c| {
        // c is a str slice
        io::println(fmt!("%s", c))
    }
}
```
Output:
```
bytes: 5, chars: 3, clusters: 2
ü
ü
```
