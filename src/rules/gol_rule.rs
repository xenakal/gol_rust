use crate::context::gol_context;

// Interface defining the transition rules for the CA
pub trait GolRule {
    fn apply_rules<T: gol_context::GolContext>(&self, gol_context: &mut T) -> ();
}

