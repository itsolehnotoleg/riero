// main file for R-iero

mod console; // loads console.rs as module
mod font;    // loads font.rs as module
mod world;   // loads world.rs as module

use console::Console;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, Window, WindowOptions};
use world::{Material, World};

const WIDTH: usize = 320;  // world width in cells
const HEIGHT: usize = 200; // world height i cells

struct Editor{
    selected: Material,
    brush: i32,
    paused: bool,
    step_once: bool,
}

fn paint(world: &mut World, cx: i32, cy: i32, radius: i32, material: Material){
    for dy in -radius..=radius{
        for dx in -radius..=radius{
	    if dx * dx + dy * dy > radius * radius{
	        continue;
	    }
	    let x = cx + dx;
	    let y = cy + dy;

	    if material == Material::Air{
	        world.set(x, y, Material::Air);
	    }
	    else if world.is_empty(x, y){
                if !material.moves() || fastrand::u8(0..4) == 0{
                   world.set(x, y, material);
		}
	    }
	}
    }
}

// now it takes Editor instead of separate values
fn update_title(window: &mut Window, editor: &Editor){
    let title = format!(
         "R-iero | {} | brush {} | ~ or /: console",
	 editor.selected.name(),
	 editor.brush
    );
    window.set_title(&title);
}

// this function runs one command and returns answer
fn run_command(line: &str, world: &mut World, editor: &mut Editor) -> String{
    let line = line.trim_start_matches('/'); // ex: brush 5 are same as /brush 5
    let mut words = line.split_whitespace();
    let command = words.next().unwrap_or("").to_lowercase();
    let arg = words.next();

    match command.as_str(){
        "help" => "HELP CLEAR SELECT <MAT> BRUSH <N> PAUSE STEP  DEV: FILL <MAT>".to_string(),

	"clear" => {
            world.reset();
	    "WORLD RESET".to_string()
	}
	"select" => match arg.and_then(Material::from_name){
            Some(material) => {
                editor.selected = material;
		format!("SELECTED {}", material.name())
            }
	    None => "USAGE: SELECT SAND / WATER / DIRT".to_string(),
	},
	"brush" => match arg.and_then(|a| a.parse::<i32>().ok()){
            Some(n) =>{
                editor.brush = n.clamp(1, 20);
		format!("BRUSH {}", editor.brush)
	    }
	    None => "USAGE: BRUSH <1-20>".to_string(),
	},
	"pause" => {
            editor.paused = !editor.paused;
	    if editor.paused{"PAUSED".to_string()} else{"RESUMED".to_string()}
	}
	"step" => {
            editor.step_once = true;
	    "STEP".to_string()
	}
        #[cfg(feature = "dev")]
	"fill" => match arg.and_then(Material::from_name){
            Some(material) => {
                world.fill_air(material);
		format!("FILLED WITH {}", material.name())
            }
	    None => "USAGE: FILL <MATERIAL>".to_string(),
	},
	other => format!("UNKNOWN COMMAND: {}", other),
    }
}

fn main(){
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    // world simulation
    let mut world = World::new(WIDTH, HEIGHT);
    let mut window = Window::new(
        "R-iero",
	WIDTH,
	HEIGHT,
	WindowOptions{
	    scale: Scale::X4,
	    ..WindowOptions::default()
	},
    )
    .expect("Could not create window");
    
    window.set_target_fps(60);

    let mut editor = Editor{
        selected: Material::Sand,
	brush: 3,
	paused: false,
	step_once: false,
    };
    let mut console = Console::new();

    update_title(&mut window, &editor);

    while window.is_open(){
        // if console is open
	if console.open{
            if window.is_key_pressed(Key::Escape, KeyRepeat::No)
	        || window.is_key_pressed(Key::Backquote, KeyRepeat::No)
        {
	        console.open = false;
	}
	else{
            for key in window.get_keys_pressed(KeyRepeat::Yes){
                match key{
                    Key::Enter => {
                        if let Some(line) = console.submit(){
                            let answer = run_command(&line, &mut world, &mut editor);
			    console.print(&answer);
			    update_title(&mut window, &editor);
			}
		    }
		    Key::Backspace => console.backspace(),
		    _ => {

		        if let Some(c) = console::key_to_char(key){
                            console.type_char(c);
			}
		    }
		}
	    }
	}
	}
	// if console is closed
	else{
            if window.is_key_pressed(Key::Escape, KeyRepeat::No){
                break;
	    }
	    if window.is_key_pressed(Key::Backquote, KeyRepeat::No)
                || window.is_key_pressed(Key::Slash, KeyRepeat::No)
	    {
                console.open = true;
	    }
	    let mut changed = false;
	    let hotkeys = [
                (Key::Key1, Material::Sand),
		(Key::Key2, Material::Water),
		(Key::Key3, Material::Dirt),
	    ];
	    for (key, material) in hotkeys{
                if window.is_key_pressed(key, KeyRepeat::No){
                    editor.selected = material;
		    changed = true;
		}
	    }
	    if window.is_key_pressed(Key::RightBracket, KeyRepeat::Yes){
                editor.brush = (editor.brush + 1).min(20);
		changed = true;
	    }
	    if window.is_key_pressed(Key::LeftBracket, KeyRepeat::Yes){
                editor.brush = (editor.brush - 1).max(1);
		changed = true;
	    }
	    if changed{
                update_title(&mut window, &editor);
	    }
	    if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard){
                let cx = mx as i32;
		let cy = my as i32;

		if window.get_mouse_down(MouseButton::Left){
                    paint(&mut world, cx, cy, editor.brush, editor.selected);
		}
		if window.get_mouse_down(MouseButton::Right){
                    paint(&mut world, cx, cy, editor.brush, Material::Air);
		}
	    }
	}
	// simulation
	if !editor.paused || editor.step_once{
            world.update();
	    editor.step_once = false;
	}
	world.draw(&mut buffer);

	if console.open{
            console.draw(&mut buffer, WIDTH, HEIGHT);
	}
	else{
            let pause_text = if editor.paused{"  PAUSED"} else{""};
	    let hud = format!("{}  BRUSH {}{}", editor.selected.name(), editor.brush, pause_text);
            font::darken_rect(&mut buffer, WIDTH, HEIGHT, 0, 0, font::text_width(&hud) + 3, font::GLYPH_H + 4);
            font::draw_text(&mut buffer, WIDTH, HEIGHT, 2, 2, &hud, 0xFFFFFF);
	}
        window
	    .update_with_buffer(&buffer, WIDTH, HEIGHT)
	    .expect("Could not update window")
    }
}


















 
