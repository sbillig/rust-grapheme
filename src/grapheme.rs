#![feature(globs)]
#![crate_id(name = "grapheme#0.1.1")]
#![crate_type = "lib"]

use std::str::*;
mod char_classes;

pub struct Graphemes<'a> {
  gl:     &'a str,
  len:    uint,
  class:  char_classes::CharClass,
  pos:    uint,
  startp: uint,
}

impl<'a> Iterator<&'a str> for Graphemes<'a> {
  fn next(&mut self) -> Option<&'a str> {
    use char_classes::*;
    use std::str::raw::slice_bytes;

    /*
    println!("Iterating..");
    println!("  self.gl     == \"{}\"", self.gl);
    println!("  self.len    == {}", self.len);
    println!("  self.class  == {}", self.class);
    println!("  self.pos    == {}", self.pos);
    println!("  self.startp == {}", self.startp);
    */

    if self.startp >= self.len {
      return None;
    }

    let mut prev_startp = self.startp;
    while self.pos < self.len {
      let prev_class  = self.class;
      let prev_pos    = self.pos;
      prev_startp     = self.startp;

      let CharRange {ch, next} = self.gl.char_range_at(self.pos);
      self.class = char_class(ch);
      self.pos   = next;

      if break_between(prev_class, self.class) {
        self.startp = prev_pos;
        return Some(unsafe {
          slice_bytes(self.gl, prev_startp, prev_pos)
        });
      }
    }
    self.startp = self.len;
    Some(unsafe {
      slice_bytes(self.gl, prev_startp, self.len)
    })
  }
}

pub trait GraphemeList {
  fn graphemes<'a>(&'a self) -> Graphemes<'a>;
}

impl<T: std::str::Str> GraphemeList for T {
  fn graphemes<'a>(&'a self) -> Graphemes<'a> {
    let len = self.as_slice().len();
    if len == 0 {
      return Graphemes {
        gl:     self.as_slice(),
        len:    0,
        class:  char_classes::Other,
        pos:    0,
        startp: 0,
      };
    };
    let CharRange {ch, next} = self.as_slice().char_range_at(0);
    Graphemes {
      gl:     self.as_slice(),
      len:    len,
      class:  char_classes::char_class(ch),
      pos:    next,
      startp: 0,
    }
  }
}

#[cfg(test)]
mod tests {
    use std::str::*;
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
        for chunks in TESTS.iter() {
            let s = chunks.concat();
            let mut i = 0u;
            for clus in s.graphemes() {
                println!("chunks[{}]: {}", i, chunks[i]);
                println!("clus: {}", clus);
                assert!(clus == chunks[i]);
                i += 1;
            }
        }
    }

    #[test]
    fn test_cluster_count() {
        for chunks in TESTS.iter() {
            let s = chunks.concat();
            println!("s: \"{}\" [\"{}\"]", s, s.escape_default())
            let c = s.graphemes().len();
            if s.len() == 0 {
                println!("c: {}", c);
                assert!(c == 0);
            } else {
                println!("c: {}", c);
                println!("chunks.len(): {}", chunks.len());
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
