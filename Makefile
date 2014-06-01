all: src/*.rs
	rustc --crate-type=lib src/grapheme.rs

test: src/*.rs
	rustc --crate-type=lib src/grapheme.rs --test -o test
	./test

clean:
	rm -f *.rlib

.PHONY: clean

