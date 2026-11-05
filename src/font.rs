// font.rs is file that created 3x5 pixel font for R-iero
// drawn inside screen buffer

pub const GLYPH_H: usize = 5; // height of every character in pixels
const ADVANCE: usize = 4;     // 3 pixels of letter + 1 pixel of space

// every character as pixel art:
// '#' = pixel on, '.' = pixel off

// each character has 5 rows, each row is 3 pixels wide
const GLYPHS: &[(char, [&str; 5])] = &[
    ('A', [".#.", "#.#", "###", "#.#", "#.#"]),
    ('B', ["##.", "#.#", "##.", "#.#", "##."]),
    ('C', [".##", "#..", "#..", "#..", ".##"]),
    ('D', ["##.", "#.#", "#.#", "#.#", "##."]),
    ('E', ["###", "#..", "##.", "#..", "###"]),
    ('F', ["###", "#..", "##.", "#..", "#.."]),
    ('G', [".##", "#..", "#.#", "#.#", ".##"]),
    ('H', ["#.#", "#.#", "###", "#.#", ".##"]),
    ('I', ["###", ".#.", ".#.", ".#.", "###"]),
    ('J', ["..#", "..#", "..#", "#.#", ".#."]),
    ('K', ["#.#", "#.#", "##.", "#.#", "#.#"]),
    ('L', ["#..", "#..", "#..", "#..", "###"]),
    ('M', ["#.#", "###", "###", "#.#", "#.#"]),
    ('N', ["##.", "#.#", "#.#", "#.#", "#.#"]),
    ('O', [".#.", "#.#", "#.#", "#.#", ".#."]),
    ('P', ["##.", "#.#", "##.", "#..", "#.."]),
    ('Q', [".#.", "#.#", "#.#", "##.", ".##"]),
    ('R', ["##.", "#.#", "##.", "#.#", "#.#"]),
    ('S', [".##", "#..", ".#.", "..#", "##."]),
    ('T', ["###", ".#.", ".#.", ".#.", ".#."]), 
    ('U', ["#.#", "#.#", "#.#", "#.#", "###"]),
    ('V', ["#.#", "#.#", "#.#", "#.#", ".#."]),
    ('W', ["#.#", "#.#", "###", "###", "#.#"]),
    ('X', ["#.#", "#.#", ".#.", "#.#", "#.#"]), 
    ('Y', ["#.#", "#.#", ".#.", ".#.", ".#."]),
    ('Z', ["###", "..#", ".#.", "#..", "###"]),
    ('0', ["###", "#.#", "#.#", "#.#", "###"]),
    ('1', [".#.", "##.", ".#.", ".#.", "###"]), 
    ('2', ["##.", "..#", ".#.", "#..", "###"]),
    ('3', ["##.", "..#", ".#.", "..#", "##."]),
    ('4', ["#.#", "#.#", "###", "..#", "..#"]),
    ('5', ["###", "#..", "##.", "..#", "##."]), 
    ('6', [".##", "#..", "###", "#.#", "###"]),
    ('7', ["###", "..#", ".#.", ".#.", ".#."]),
    ('8', ["###", "#.#", "###", "#.#", "###"]),
    ('9', ["###", "#.#", "###", "..#", "##."]), 
    (' ', ["...", "...", "...", "...", "..."]),
    ('.', ["...", "...", "...", "...", ".#."]),
    (',', ["...", "...", "...", ".#.", "#.."]),
    (':', ["...", ".#.", "...", ".#.", "..."]), 
    ('-', ["...", "...", "###", "...", "..."]),
    ('_', ["...", "...", "...", "...", "###"]),
    ('=', ["...", "###", "...", "###", "..."]),
    ('/', ["..#", "..#", ".#.", "#..", "#.."]), 
    ('>', ["#..", ".#.", "..#", ".#.", "#.."]),
    ('!', [".#.", ".#.", ".#.", "...", ".#."]),
    ('?', ["##.", "..#", ".#.", "...", ".#."]),
    ('[', ["##.", "#..", "#..", "#..", "##."]), 
    (']', [".##", "..#", "..#", "..#", ".##"]),
];

// find pixel art for a character
// marks unknown char as '?'
fn glyph(c: char) -> [&'static str; 5]{
    let c = c.to_ascii_uppercase(); // makes 'a' and 'A' looks the same
    
    for(ch, rows) in GLYPHS{
        if *ch == c{
	    return *rows;
	}   
    }
    glyph('?')
}

// draw a line of text
// with its top-left corner at (x, y)
pub fn draw_text(buffer: &mut[u32], width: usize, height:usize,
                 x: usize, y: usize, text: &str, color: u32){
    let mut cx = x; // x position of the current character

    for c in text.chars(){
        let rows = glyph(c);
	
	for(gy, row) in rows.iter().enumarate(){
            for(gx, pixel) in row.chars().enumarate(){
                if pixel == '#'{
                    let px = cx + gx;
		    let py = y + gy;
		    
		    // draws only if pixel on screen
		    if px < width && py < height{
                        buffer[py * width + px] = color;
		    } 
		}
	    }
	}
	cx += ADVANCE; // move right for the next char
    }
}

// make rectengular part of screen darker, so text can readable
pub fn darken_rect(buffer: &mut[u32], width: usize, height: usize,
                   x: usize, y: usize, w: usize, h: usize){
    for py in y..(y + h).min(height){
        for px in x..(x + w).min(width){
	    let i = py * width + px;
	    buffer[i] = (buffer[i] >> 1) & 0x7F7F7F;
	}
    }
}

// checks how wide a piece of text
pub fn text_width(text: &str) -> usize{
    text.chars().count() * ADVANCE
}
