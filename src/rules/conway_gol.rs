use crate::rules::gol_rule::GolRule;
// Concrete implementation for GolRules
pub struct ConwayRules { }

impl ConwayRules {

    pub fn new() -> ConwayRules {
        ConwayRules{}
    }
}

impl GolRule for ConwayRules {

    fn apply_rules<T: crate::context::gol_context::GolContext>(&self, gol_context:&mut T) {

        let mut idx: usize = 0;
        let mut indices_to_flip: Vec<usize> = vec![];
        while (idx as usize)<gol_context.get_len() {

            let alive_neighbors = gol_context.number_live_neighbours(idx);
            let is_cell_alive = gol_context.is_cell_alive(idx);

            // if ((alive_neighbors == 3 && !is_cell_alive) || (is_cell_alive && !(alive_neighbors == 3 && !is_cell_alive)))
            // if ((alive_neighbors == 3 && !is_cell_alive) || (is_cell_alive && (alive_neighbors != 3 || is_cell_alive)))
            // if ((alive_neighbors == 3 && !is_cell_alive) || (is_cell_alive && (alive_neighbors != 3 || is_cell_alive)))
            // we could write all these rules in one line by combining the constrains
            if (alive_neighbors == 2 || alive_neighbors == 3) && is_cell_alive {
                // no need to change anything
            }
            else if alive_neighbors == 3 && !is_cell_alive {
                indices_to_flip.push(idx);
            }
            else if is_cell_alive {
                indices_to_flip.push(idx);
            }
            idx+=1;
        }

        // use map instead
        for idx in indices_to_flip {
            gol_context.set_state_at(!gol_context.get_state_at(idx), idx);
        }
    }
} 