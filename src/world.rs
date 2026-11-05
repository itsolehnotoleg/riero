// file that simulates world: materials, cells and grid

#[derive(Clone, Copy, PartialEq)]
pub enum Material{
    Air,
    Dirt,
    Sand,
    Water,
}

// functions for Material
impl Material{
    // checks if material moves on its own
    pub fn moves(self) -> bool{
        matches!(self, Material::Sand | Material::Water)
    }

    // title for window, so humans can read it
    pub fn name(self) -> &'static str{
        match self{
	    Material::Air => "Air",
	    Material::Dirt => "Dirt",
	    Material::Sand => "Sand",
	    Material::Water => "Water",
	}
    }
}

// one pixel of the world
#[derive(Clone, Copy)]
pub struct Cell{
    pub material: Material, // what this cell is made of
    pub shade: u8,          // random darkness, so materials look grainy
    pub updated: bool,      // checks if cells moved this frame
}

impl Cell{
    // create a new cell of the given material with a random shade
    pub fn new(material: Material) -> Cell{
        Cell{
            material,
	    shade: fastrand::u8(0..30),
	    updated: false,  // no they did NOT!
	}
    }

    // turn this cell into a screen color (0xRRGGBB)
    pub fn color(&self) -> u32{
	// pick the base color for each material
	let(r, g, b): (u8, u8, u8) = match self.material{
	    Material::Air => return 0x87CEED, // sky: no grain, return right away
            Material::Dirt => (130, 90, 43),
	    Material::Sand => (230, 200, 120),
	    Material::Water => return 0x2860C8, // water: blue, no grain
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
    // counts how many updates happened
    frame: u64,
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
	World{width, height, cells, frame: 0}
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

    // check where material is on x and y coordinates
    // none if outside the world
    fn material_at(&self, x: i32, y:i32) -> Option<Material>{
        if self.in_bounds(x, y){
	    Some(self.cells[self.index(x as usize, y as usize)].material)
	}
	else{
	    None
	}
    }

    // is (x, y) inside the world AND air?
    // outside the world counts as a wall
    pub fn is_empty(&self, x: i32, y: i32) -> bool{
        self.in_bounds(x, y)
	    && self.cells[self.index(x as usize, y as usize)].material == Material::Air
    }

    // checks is sand moves into x and y coordinates
    // sand falls through air AND sinks through water
    // TODO: sand interacts with water and creates new material
    fn sand_can_enter(&self, x: i32, y: i32) -> bool{
        matches!(
	    self.material_at(x, y),
	    Some(Material::Air) | Some(Material::Water)
	)
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
    fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32){
        let a = self.index(x1 as usize, y1 as usize);
	let b = self.index(x2 as usize, y2 as usize);
	self.cells.swap(a, b);
	self.cells[b].updated = true;
    }

    // run one step of physics. called once per frame
    pub fn update(&mut self){
	// start of frame, nothing has moved
	for cell in self.cells.iter_mut(){
	    cell.updated = false;
	}

	self.frame = self.frame.wrapping_add(1);
	let left_to_right = self.frame % 2 == 0;
	let w = self.width as i32;

        // goes bottom to top
	// falling grain moves only once per frame
	for y in(0..self.height as i32).rev(){
            for step in 0..w{
		let x = if left_to_right{step} else{w - 1 - step};
	        let i = self.index(x as usize, y as usize);
		
 		// if cell moved we skip it
		// then asset rule of the material
		if self.cells[i].updated{
		    continue;
		}
		match self.cells[i].material{
		    Material::Sand => self.update_sand(x, y),
		    Material::Water => self.update_water(x, y),
		    // unmovable materials
		    Material::Air | Material::Dirt => {} 
		}
	    }
	}
    }

    // sand rules:
    // 1. fall down if blocked. 2. slide diagonally if block, rest
    fn update_sand(&mut self, x: i32, y: i32){
	// rule 1: fall down
	if self.sand_can_enter(x, y + 1){
            self.swap(x, y, x, y + 1);
	    return;
	}
	
	// coin flip
	let dir = if fastrand::bool(){1} else{-1};

	// rule 2: slide diagonally
	if self.sand_can_enter(x + dir, y + 1){
	    self.swap(x, y, x + dir, y + 1);
	}
	else if self.sand_can_enter(x - dir, y + 1){
	    self.swap(x, y, x - dir, y + 1);
	}
	// note: grain rests if both diagonals are blocked
	// forming a pile from it
    }

    // water rules:
    // 1. fall straight down. 2. fall diagonally. 3. unlike sand it slide sideways
    fn update_water(&mut self, x: i32, y: i32){
        // rule 1: fall down
	if self.is_empty(x, y + 1){
	    self.swap(x, y, x, y + 1);
	    return;
	}
        
        let dir = if fastrand::bool(){1} else{-1};

        // rule 2: fall diagonally (rand)
	if self.is_empty(x + dir, y + 1){
	    self.swap(x, y, x + dir, y + 1);
	    return;
	}
	if self.is_empty(x - dir, y + 1){
	    self.swap(x, y, x - dir, y + 1);
	    return;
	}

	// rule 3: can't fall at all so it goes side ways
	if self.is_empty(x + dir, y){
	    self.swap(x, y, x + dir, y);
	}
	else if self.is_empty(x - dir, y){
	    self.swap(x, y, x - dir, y);
	}
    }
 
    // screen's buffer copies color of ever cell
    // (they both same layout, cell i = pixel i)
    pub fn draw(&self, buffer: &mut[u32]){
        for(i, cell) in self.cells.iter().enumerate(){   
	    buffer[i] = cell.color();
        }
    }
}
