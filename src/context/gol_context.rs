// Interface defining an object that has a set of boolean cells positioned relative to each other. 
// It allows to define a context on which to apply the transition rules. 
pub trait GolContext {

    // Should a new() func be included here ? 

    fn get_len(&self) -> usize;

    fn update_view(&self) -> ();

    fn number_live_neighbours(&self, inde: usize) -> usize;

    fn is_cell_alive(&self, index: usize) -> bool;

    fn set_state_at(&mut self, state: bool, index: usize) -> ();

    fn get_state_at(&self, index: usize) -> bool;

    fn print_details(&self) -> ();
}
