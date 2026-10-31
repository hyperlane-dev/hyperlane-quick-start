/// The characters that end a word of a line.
///
/// The separators stay attached to the word they follow, so the segments of a
/// word diff still add up to the text they were cut from.
pub(crate) const WORD_BOUNDARIES: &[char] = &[' ', '\t', '\r', '\n'];

/// The number of the slots the Myers table adds for every unit of the length
/// of the two compared sequences: the table holds one slot per diagonal, and
/// there are `old + new` diagonals on each side of its centre.
pub(crate) const TABLE_SLOTS_PER_UNIT: usize = 2;

/// The largest number of the diagonals the Myers algorithm walks.
///
/// The search of the algorithm is quadratic in the worst case, so the walk is
/// cut off at this depth and the two texts are reported as one removal and
/// one addition.
pub(crate) const MAX_DIAGONALS: usize = 8192;
