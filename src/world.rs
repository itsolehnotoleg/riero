// file that simulates world: materials, cells and grid

#[derive(Clone, Copy, PartialEq)]
pub enum Material{
    Air,
    Dirt,
    Sand,
}

// one pixel of the world
#[derive(Clone, Copy, PartialEq)]
pub struct Cell{
    pub material: Material, // what this cell is made of
    pub shade: u8,          // random darkness, so materials look grainy
}

impl Cell{
    // create a new cell of the given material with a random shade
    pub fn new(material: Material) -> Cell{
        Cell{
            material,
	    shade: fastrand::u8(0..30),
	}
    }

    // turn this cell into a screen color (0xRRGGBB)
    pub fn color(&self) -> u32{
	// pick the base color for each material
	let(r, g, b): (u8, u8, u8) = match self.material{
	    Material::Air => return 0x87CEED, // sky: no grain, return right away
            Material::Dirt => (130, 90, 43),
	    Material::Sand => (230, 200, 120),
	};

	// darken by this cell's shade. 
	// saturating_sub stops at 0 instead of crashing
        let r = r.saturating_sub(self.shade);
	let g = g.saturating_sub(self.shade);
	let b = b.saturating_sub(self.shade);

	// pack red, green, blue into one number: 0xRRGGBB
	((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }
}

// the whole world:
// a grid of cells stored by row in one list
pub struct World{
    pub width: usize,
    pub height: usize,
    // private: other files must use our functions to change it
    cells: Vec<Cell>,
}

impl World{
    // create a new world: air on top, dirt on the bottom
    pub fn new(width: usize, height: usize) -> World{
        // reserve memory for every cell up front
        let mut cells = Vec::with_capacity(width * height);

	for y in 0..height{
	    for _x in 0..width{
                let material = if y > 140{Material::Dirt} else{Material::Air};
		cells.push(Cell::new(material));
	    }
	}
	World{width, height, cells}
    }
    
    // convert (x, y) into a position in the cells list
    fn index(&self, x: usize, y: usize) -> usize{
        y * self.width + x
    }

    // is (x, y) inside the world?
    // uses i32 so negative positions are safe to check
    fn in_bounds(&self, x: i32, y: i32) -> bool{
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    // is (x, y) inside the world AND air?
    // outside the world counts as a wall
    pub fn is_empty(&self, x: i32, y: i32) -> bool{
        self.in_bounds(x, y)
	    && self.cells[self.index(x as usize, y as usize)].material == Material::Air
    }

    // place new cell of material at (x, y)
    // ignores positions outside the world
    pub fn set(&mut self, x: i32, y: i32, material: Material){
        if self.in_bounds(x, y){
	    let i = self.index(x as usize, y as usize);
	    self.cells[i] = Cell::new(material);
	} 
    }
    // swap two cells. a grain trades places with air
    fn swap(&mut self, x1: i32, y1: i32, y2: i32){
        let a = self.index(x1 as usize, y1 as usize);
	let b = self.index(x2 as usize, y2 as usize);
	self.cells.swap(a, b);
    }

    // run one step of physics. called once per frame
    pub fn update(&mut self){
        // goes bottom to top
	// falling grain moves only once per frame
	for y in(0..self.height as i32).rev(){
            for x in 0..self.width  as 132{
	        let i = self.index(x as usize, y as usize);
		if self.cells[i].material == Material::Sand{
	            self.update_sand(x, y);
		}
	    }
	}
    }

    // sand rules:
    // 1. fall down if blocked. 2. slide diagonally if block, rest
    fn update_sand(&mut self, x: i32, y: i32){
        // rule 1: if there space, sand is falls down
	if self.is_empty(x, y + 1){
            self.swap(x, y, x, y + 1);
	    return;
	}
	
	// coin flip
	let dir = if fastrand::bool(){1} else{-1};

	// rule 2: slide down to the one diagonal
	// rule 3: or other diagonal
	if self.is_empty(x + dir, y + 1){
	    self.swap(x, y, x + dir, y + 1);
	}
	else if self.is_empty(x - dir, y + 1){
	    self.swap(x, y, x - dir, y + 1);
	}
	// note: grain rests if both diagonals are blocked
	// forming a pile from it
    }

    // screen's buffer copies color of ever cell
    // (they both same layout, cell i = pixel i)
    pub fn draw(&self, buffer: &mut[u32]){
        for(i, cell) in self.cells.iter().enumerate(){   
	    buffer[i] = cell.color();
        }
    }
}
