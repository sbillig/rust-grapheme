#[link(name = "grapheme",
       vers = "0.1.0",
       uuid = "a91f4949-e26e-4822-ba82-bfc468d6de77",
       url  = "https://github.com/sbillig/rust-grapheme")];

#[crate_type = "lib"];

use core::str::*;
mod char_classes;

pub fn cluster_count(s: &str) -> uint {
    use char_classes::*;

    let len = s.len();
    if len == 0 {
        return 0;
    }

    let CharRange {ch, next} = char_range_at(s, 0);

    let mut left = char_class(ch);
    let mut pos = next;
    let mut count = 1u;

    while pos < len {
        let CharRange {ch, next} = char_range_at(s, pos);
        let right = char_class(ch);

        if break_between(left, right) {
            count += 1;
        }

        left = right;
        pos = next;
    }
    count
}


#[cfg(test)]
mod tests {
    use char_classes::*;

/*
    #[test]
    fn test_cluster_count() {
        fn t(s: &str, c: uint) {
            assert!(cluster_count(s) == c);
        }
        t(~"", 0);
        t(~"hello", 5);
        t(~"a\U0001f1e6b", 3);
        t(~"\U0001f1f7\U0001f1fa", 1);
        t(~"\U0001f1f7\U0001f1fa\U0001f1f8", 1);
        t(~"\U0001f1f7\U0001f1fa\U0001f1f8\U0001f1ea", 1);
        t(~"\U0001f1f7\U0001f1fa\u200b\U0001f1f8\U0001f1ea", 3);
        t(~"\u0020\u200d\u0646", 2);
    }
*/
    #[test]
    fn blah() {
        assert!(is_spacing_mark('\u09cc'));
        assert!(char_class('\u09cc') == SpacingMark);
        assert!(char_class('\U0001f1e6') == RegionalIndicator);
    }

    #[test]
    fn f() {
    }
}
