
#[deriving(Eq)]
pub enum CharClass {
    // These are ordered to match the table on [2]
    Other,
    CR,
    LF,
    Control,
    Extend,
    SM, // Spacing_Mark
    L,
    V,
    T,
    LV,
    LVT,
    RI  // Regional_Indicator
}
