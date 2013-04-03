#[link(name = "grapheme",
       vers = "0.1.0",
       uuid = "a91f4949-e26e-4822-ba82-bfc468d6de77",
       url  = "https://github.com/sbillig/rust-grapheme")];

#[crate_type = "lib"];

mod char_classes;

#[test]
fn blah() {
    use char_classes::*;
    assert!(is_spacing_mark('\u09cc'));
    assert!(char_class('\u09cc') == SpacingMark);
}
