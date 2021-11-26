// use self::gol_context::GolContext; DOESNT WORK??
use crate::context::gol_context::GolContext;

// Concrete implementation for GolContext
pub struct Basic2DConsoleBoard {
    width: usize,
    height: usize, 
    current_state: Vec<bool>,
}

impl Basic2DConsoleBoard {

    pub fn new(init_state:Vec<bool>, height:usize, width:usize) -> Basic2DConsoleBoard {
        Basic2DConsoleBoard {
            width, 
            height,
            current_state: init_state,
        }
    }

    fn get_width(&self) -> usize { self.width }

    fn get_height(&self) -> usize { self.height }

    fn get_indices_of_neighbours(&self, index: usize) -> [usize; 8] {

        let w = self.get_width() as i32;
        let h = self.get_height() as i32;
        let idx = index as i32;

        let mut neighbours_indices = [(h*w) as usize; 8];

        let mut curr_nei = 0;
        for i in [-1,0,1] {
            for j in [-1,0,1] {
                if i==0 && j==0 { continue; }
                let nei_idx = idx + w*j + i;
                if nei_idx >= 0 && nei_idx < w*h {
                    // don't allow wrap around
                    if !((idx < w && j == -1) || (idx > (h-1)*w && j == 1) || (idx % w == 0 && i == -1) || ((idx+1) % w == 0 && i == 1)) {
                        neighbours_indices[curr_nei] = nei_idx as usize;
                    }
                }
                curr_nei += 1;
            }
        }
        neighbours_indices
    }

    fn print_board(&self) {
        let mut idx = 0;
        while idx < self.get_len() {
            if self.is_cell_alive(idx) {
                print!(" x ");
            }
            else {
                print!(" _ ");
            }
            if (idx+1) % self.width == 0 {
                println!("");
            }
            idx += 1;
        }
        println!("\n");
    }
}

impl GolContext for Basic2DConsoleBoard {
    // define trait that converts Self into Vec<bool> and use generic param in new

    fn get_len(&self) -> usize { self.get_width()*self.get_height() }

    fn update_view(&self) {
        self.print_board();
    }

    fn number_live_neighbours(&self, index: usize) -> usize {

        let indices_neighbours = self.get_indices_of_neighbours(index);
        // TODO: how to do this with some kind of list.filter(_ => _).count() equivalent? 
        // indices_neighbours.into_iter().filter(|idx| idx != self.height*self.width).count()
        let mut alive = 0;
        for idx_nei in indices_neighbours {
            if idx_nei != self.height*self.width && self.is_cell_alive(idx_nei) {
                alive += 1;
            }
        }
        alive
    }

    fn is_cell_alive(&self, index: usize) -> bool { self.current_state[index] }

    fn set_state_at(&mut self, state: bool, index: usize) {
        if index > 0 && index < self.height*self.width {
            self.current_state[index] = state;
        }
    }

    fn get_state_at(&self, index: usize) -> bool {
        self.current_state[index]
    }

    fn print_details(&self) {
        println!("----width = {0}, height = {1}", self.width, self.height);
        println!("----{:?}", self.current_state);
    }


} 