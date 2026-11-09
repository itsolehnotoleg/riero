// console.rs file for in-game console
// it stores typed text and output, and draws itself

use crate::font;
use minifb::Key;

const VISIBLE_LINES: usize = 8;          // how many output lines are shown
const LINE_H: usize = font::GLYPH_H + 2; // line height: letter +  2 pixels gap
const MAX_LINES: usize = 100;            // older lines are forgotten after this
const MAX_INPUT: usize = 70;             // longest command you can type

pub struct Console{
    pub open: bool,     // checks if console is visible
    input: String,      // checks the input
    lines: Vec<String>, // past output, oldest first
}

impl Console{
    pub fn new() -> Console{
        Console{
            open: false,
	    input: String::new(),
	    lines: vec!["R-IERO CONSOLE. TYPE HELP".to_string()],
	}
    }

    // add a line of output. if there overflow it clears it up
    pub fn print(&mut self, text: &str){
        self.lines.push(text.to_string());
	if self.lines.len() > MAX_LINES{
	    self.lines.remove(0);
	}
    }

    // add one typed char to the input
    pub fn type_char(&mut self, c: char){
        if self.input.len() < MAX_INPUT{
            self.input.push(c);
	}
    }

    // delete the last typed char
    pub fn backspace(&mut self){
        self.input.pop();
    }

    // if enter was pressed it echo typed line
    // return none if nothing was typed
    pub fn submit(&mut self) -> Option<String>{
        let line = self.input.trim().to_string();
	self.input.clear();

	if line.is_empty(){
            return None;
	}
	self.print(&format!("> {}", line));
	Some(line)
    }

    // this function draw console on the top of the screen
    pub fn draw(&self, buffer: &mut[u32], width: usize, height: usize){
        // dark background
	let panel_h = (VISIBLE_LINES + 1) * LINE_H + 4;
	font::darken_rect(buffer, width, height, 0, 0, width, panel_h);
	font::darken_rect(buffer, width, height, 0, 0, width, panel_h);
        
	let start = self.lines.len().saturating_sub(VISIBLE_LINES);
	
	for (row, line) in self.lines[start..].iter().enumerate(){
            font::draw_text(buffer, width, height, 2, 2 + row * LINE_H, line, 0xCCCCCC);
	}
	let input_y = 2 + VISIBLE_LINES * LINE_H;
	let input_text = format!("> {}_", self.input);
	font::draw_text(buffer, width, height, 2, input_y, &input_text, 0xFFFF66);
    }
}

pub fn key_to_char(key: Key) -> Option<char>{
    let c = match key{
        Key::A => 'a', Key::B => 'b', Key::C => 'c', Key::D => 'd',
	Key::E => 'e', Key::F => 'f', Key::G => 'g', Key::H => 'h',
	Key::I => 'i', Key::J => 'j', Key::K => 'k', Key::L => 'l',
	Key::M => 'm', Key::N => 'n', Key::O => 'o', Key::P => 'p',
	Key::Q => 'q', Key::R => 'r', Key::S => 's', Key::T => 't',
	Key::U => 'u', Key::V => 'v', Key::W => 'w', Key::X => 'x',
	Key::Y => 'y', Key::Z => 'z',
	
	Key::Key0 => '0', Key::Key1 => '1', Key::Key2 => '2', Key::Key3 => '3',
        Key::Key4 => '4', Key::Key5 => '5', Key::Key6 => '6', Key::Key7 => '7',
	Key::Key8 => '8', Key::Key9 => '9',

        Key::Space => ' ',
	Key::Minus => '-',
	Key::Period => '.',
	Key::Comma => ',',
	Key::Slash => '/',
	Key::Equal => '=',
	_ => return None,
    };
    Some(c)
}
