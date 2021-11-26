use crate::context::gol_context;
use crate::rules::gol_rule;

pub struct SimpleRunner { }

// impl<T: gol_context::GolContext, U: gol_rule::GolRule> SimpleRunner<T, U> {
impl SimpleRunner {

    pub fn new() -> SimpleRunner {
        SimpleRunner { }
    }

    pub fn run<T: gol_context::GolContext, U: gol_rule::GolRule>(&self, context: &mut T, rules: U) {
        self.iterate(10, context, rules);
    }

    fn iterate<T: gol_context::GolContext, U: gol_rule::GolRule>(&self, iters: usize, context: &mut T, rules: U) {
        if iters > 0 {
            context.update_view();
            rules.apply_rules(context);
            self.iterate(iters-1, context, rules);
        }
    }
}



