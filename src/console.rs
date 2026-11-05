// console.rs file for in-game console
// it stores typed text and output, and draws itself

use crate::font;
use minifb::Key;

const VISIBLE_LINES: usize = 8;          // how many output lines are shown
const LINE_H: usize = font::GLYPH_H + 2; // line height: letter +  2 pixels gap

