use crate::browser;

pub enum Draw {
    Diamond,
    MiddleLine,
}

pub struct DrawLogo;

impl Draw {
    pub fn draw(&self) {
        let context = browser::context().unwrap();

        match self {
            Draw::Diamond => DrawLogo::draw_diamond(&context),
            Draw::MiddleLine => DrawLogo::draw_middle_line(&context),
        }
    }
}

impl DrawLogo {
    pub fn draw_diamond(context: &web_sys::CanvasRenderingContext2d) {
        context.set_line_width(5.0);
        context.begin_path();
        context.move_to(300.0, 50.0);
        context.line_to(50.0, 300.0);
        context.line_to(300.0, 550.0);
        context.line_to(550.0, 300.0);
        context.line_to(300.0, 50.0);
        context.close_path();
        context.stroke();
    }

    pub fn draw_middle_line(context: &web_sys::CanvasRenderingContext2d) {
        context.set_line_width(5.0);
        context.begin_path();
        context.move_to(175.0, 175.0);
        context.line_to(425.0, 425.0);
        context.stroke();
    }
}
