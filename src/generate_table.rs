/* References:

[0] State machine description:
http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries

[1] Character classes:
http://www.unicode.org/Public/UNIDATA/auxiliary/GraphemeBreakProperty.txt

[2] The generated lookup matrix should match this table:
http://www.unicode.org/Public/UNIDATA/auxiliary/GraphemeBreakTest.html

NOTE: At the time of writing, there aren't any characters in the Prepend class
(see table 2 in [0]), so the Prepend class and its associated rule (GB9b) aren't
considered here.
*/

mod char_classes;

fn main() {
    use char_classes::*;

    let mut sv: LookupTable = [[false,..CLASS_CNT],..CLASS_CNT];

    let RI = RegionalIndicator;
    let SM = SpacingMark;

    let control = &[Control, CR, LF];
    let any = &[Extend, RI, SM,
                L, V, T, LV, LVT,
                Other];

    let join  = |from, to| { write(true,  &mut sv, from, to) };
    let split = |from, to| { write(false, &mut sv, from, to) };

    // GB1,2 are 'break at start and end of text'
    join(&[CR],    &[LF]);         // GB3
    split(control, any);           // GB4
    split(any,     control);       // GB5
    join(&[L],     &[L,V,LV,LVT]); // GB6
    join(&[LV,V],  &[V,T]);        // GB7
    join(&[LVT,T], &[T]);          // GB8
    join(&[RI],    &[RI]);         // GB8a
    join(any,      &[Extend, SM]); // GB9,9a
    // GB10 is satisfied by default 'false' value in sv

    print_static_matrix(&sv);

    fn write(val: bool, sv: &mut LookupTable, from: &[CharClass], to: &[CharClass]) {
        for from.each |&f| {
            for to.each |&t| {
                sv[f as int][t as int] = val;
            }
        }
    }
    fn print_static_matrix(sv: &LookupTable) {
        print("[");
        for sv.eachi |i, &row| {
            if i != 0 {
                print(" ");
            }
            print(fmt!("%?", row));
            if i < sv.len()-1 {
                print(",\n");
            }
        }
        print("]\n");
    }
}
