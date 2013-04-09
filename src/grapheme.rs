#[link(name = "grapheme",
       vers = "0.1.0",
       uuid = "a91f4949-e26e-4822-ba82-bfc468d6de77",
       url  = "https://github.com/sbillig/rust-grapheme")];

#[crate_type = "lib"];

use core::str::*;
mod char_classes;

pub fn cluster_count(s: &str) -> uint {
    let mut c = 0u;
    do each_cluster(s) |_| { c += 1; true }
    c
}

pub fn each_cluster<'a>(s: &'a str, it: &fn(&'a /*'*/ str) -> bool) {
    use char_classes::*;
    use core::str::raw::slice_bytes;

    let len = s.len();
    if len == 0 {
        return;
    }

    let CharRange {ch, next} = char_range_at(s, 0);

    let mut left = char_class(ch),
            pos = next,
            startp = 0;

    while pos < len {
        let CharRange {ch, next} = char_range_at(s, pos);
        let right = char_class(ch);

        if break_between(left, right) {
            if !it(unsafe{slice_bytes(s, startp, pos)}) { return; }
            startp = pos;
        }

        left = right;
        pos = next;
    }
    it(unsafe{slice_bytes(s, startp, len)});
}

#[cfg(test)]
mod tests {
    use core::str::*;
    use char_classes::*;
    use super::*;

    static TESTS: [&'static [&'static str], ..16] = [
        &[""],
        &["x"],
        &["a","b"],
        &["a\u030a","a\u030a\u030a\u030a"],
        &["\u1100\u1161\u11a8"],
        &["\u0ba8\u0bbf"],

        //unicode.org/Public/UNIDATA/auxiliary/GraphemeBreakTest.html#s1
        &["a","\U0001f1e6","b"],
        &["\U0001f1f7\U0001f1fa"],
        &["\U0001f1f7\U0001f1fa\U0001f1f8"],
        &["\U0001f1f7\U0001f1fa\U0001f1f8\U0001f1ea"],
        &["\U0001f1f7\U0001f1fa","\u200b","\U0001f1f8\U0001f1ea"],
        &["\U0001f1e6\U0001f1e7\U0001f1e8"],
        &["\U0001f1e6\u200d","\U0001f1e7\U0001f1e8"],
        &["\U0001f1e6\U0001f1e7\u200d","\U0001f1e8"],
        &[" \u200d","\u0646"],
        &["\u0646\u200d"," "]
    ];

    #[test]
    fn test_each_cluster() {
        for TESTS.each |&chunks| {
            let s = connect_slices(chunks, "");
            let mut i = 0u;
            for each_cluster(s) |clus| {
                assert!(clus == chunks[i]);
                i += 1;
            }
        }
    }

    #[test]
    fn test_cluster_count() {
        for TESTS.each |&chunks| {
            let s = connect_slices(chunks, ""),
                c = cluster_count(s);
            if s.len() == 0 {
                assert!(c == 0);
            } else {
                assert!(c == chunks.len());
            }
        }
    }
    #[test]
    fn test_classes() {
        assert!(char_class('\n') == LF);
        assert!(char_class('\r') == CR);
        assert!(char_class('\t') == Control);
        assert!(char_class('\u1100') == L);
        assert!(char_class('\u1160') == V);
        assert!(char_class('\u11A8') == T);
        assert!(char_class('\uac00') == LV);
        assert!(char_class('\uac01') == LVT);
        assert!(char_class('\u0903') == SpacingMark);
        assert!(char_class('\u0308') == Extend);
        assert!(char_class(' ') == Other);
        assert!(char_class('a') == Other);
        assert!(char_class('S') == Other);
        assert!(char_class('\U0001f1e6') == RegionalIndicator);
    }
}
