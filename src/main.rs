pub mod context;
pub mod rules;
pub mod gol_runner;

use crate::context::console_2d_board;
use crate::rules::conway_gol;
extern crate kiss3d;
extern crate nalgebra as na;

// use kiss3d::window::Window;
fn main() {

    let init_config = vec![false, true, false, false, false, true, false, false, false, true, false, false];
    let mut board = console_2d_board::Basic2DConsoleBoard::new(init_config, 3, 4);
    let rules = conway_gol::ConwayRules::new();

    let runner = gol_runner::SimpleRunner::new();
    runner.run(&mut board, rules);

}

/*
    fn main_old() {


        println!("Begin main");
        let init_config = vec![false, true, false, false, false, true, false, false, false, true, false, false];
        let mut board : gol_context::Basic2DGolBoard = gol_board::Basic2DGolBoard::new(init_config, 3, 4);

        board.update_view();
        let mut iter = 0;
        while iter < 20 {
            core_engine::iterate_conway(&mut board);
            iter += 1;
            board.update_view();
        }
        println!("End main");

        let mut window = Window::new("Game Of Life !");
        let mut c      = window.add_rectangle(800.0, 800.0);

        let (mut r, mut g, mut b): (f32, f32, f32) = (0.0, 0.0, 0.0);
        c.set_color(r, g, b);

        while window.render() {
            r = r + 0.001;
            g = g + 0.001;
            b = b + 0.001;
            c.set_color(r, g, b);
        }

    }
*/
