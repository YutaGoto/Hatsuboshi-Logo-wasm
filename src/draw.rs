use crate::browser;

pub struct Draw;
pub struct DrawLogo;

impl Draw {
    pub fn draw() {
        let context = browser::context().unwrap();

        DrawLogo::draw_diamond(&context);
        DrawLogo::draw_middle_line(&context);
        DrawLogo::draw_circle(&context);
        DrawLogo::draw_h_in_circle(&context);
        DrawLogo::draw_6lines(&context);
        DrawLogo::curved_sparkle(&context);
        DrawLogo::draw_arc(&context);
        DrawLogo::draw_5lines_between_arc(&context);
    }
}

impl DrawLogo {
    const LINE_WIDTH: f64 = 10.0;

    pub fn draw_diamond(context: &web_sys::CanvasRenderingContext2d) {
        context.set_line_width(Self::LINE_WIDTH);
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
        context.set_line_width(Self::LINE_WIDTH);
        context.begin_path();
        context.move_to(175.0, 175.0);
        context.line_to(425.0, 425.0);
        context.stroke();
    }

    pub fn draw_circle(context: &web_sys::CanvasRenderingContext2d) {
        context.set_line_width(Self::LINE_WIDTH);
        context.begin_path();
        context
            .arc(168.0, 300.0, 78.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.stroke();
    }

    pub fn draw_h_in_circle(context: &web_sys::CanvasRenderingContext2d) {
        context.set_line_width(Self::LINE_WIDTH);
        context.begin_path();
        context.move_to(125.0, 365.0);
        context.line_to(125.0, 235.0);
        context.move_to(125.0, 300.0);
        context.line_to(205.0, 300.0);
        context.move_to(205.0, 235.0);
        context.line_to(205.0, 365.0);
        context.stroke();
    }

    pub fn draw_6lines(context: &web_sys::CanvasRenderingContext2d) {
        let adjust = 127.0;
        let gap = 23.0;

        context.set_line_width(Self::LINE_WIDTH);
        context.begin_path();
        context.move_to(405.0, 405.0);
        context.line_to(405.0 - adjust, 405.0 + adjust);

        context.move_to(405.0 - 1.0 * gap, 405.0 - 1.0 * gap);
        context.line_to(405.0 - 1.0 * gap - adjust, 405.0 - 1.0 * gap + adjust);

        context.move_to(405.0 - 2.0 * gap, 405.0 - 2.0 * gap);
        context.line_to(405.0 - 2.0 * gap - adjust, 405.0 - 2.0 * gap + adjust);

        context.move_to(405.0 - 3.0 * gap, 405.0 - 3.0 * gap);
        context.line_to(405.0 - 3.0 * gap - adjust, 405.0 - 3.0 * gap + adjust);

        context.move_to(405.0 - 4.0 * gap, 405.0 - 4.0 * gap);
        context.line_to(405.0 - 4.0 * gap - adjust, 405.0 - 4.0 * gap + adjust);

        context.move_to(405.0 - 5.0 * gap, 405.0 - 5.0 * gap);
        context.line_to(405.0 - 5.0 * gap - adjust, 405.0 - 5.0 * gap + adjust);

        context.stroke();
    }

    pub fn curved_sparkle(context: &web_sys::CanvasRenderingContext2d) {
        context.set_line_width(Self::LINE_WIDTH);
        context.set_line_join("bevel");

        context.begin_path();
        context
            .arc(
                174.0,
                48.5,
                125.5,
                std::f64::consts::PI * 0.0,
                std::f64::consts::PI * 0.5,
            )
            .unwrap();
        context
            .arc(
                174.0,
                300.5,
                125.5,
                std::f64::consts::PI * 1.5,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        context
            .arc(
                425.5,
                300.5,
                125.5,
                std::f64::consts::PI * 1.0,
                std::f64::consts::PI * 1.5,
            )
            .unwrap();
        context
            .arc(
                425.5,
                48.5,
                125.5,
                std::f64::consts::PI * 0.5,
                std::f64::consts::PI * 1.0,
            )
            .unwrap();

        context.stroke();
    }

    pub fn draw_arc(context: &web_sys::CanvasRenderingContext2d) {
        context.set_line_width(Self::LINE_WIDTH);
        context.begin_path();
        context
            .arc(
                492.0,
                365.0,
                85.0,
                std::f64::consts::PI * 0.75,
                std::f64::consts::PI * 1.75,
            )
            .unwrap();
        context.stroke();
    }

    pub fn draw_5lines_between_arc(context: &web_sys::CanvasRenderingContext2d) {
        context.set_line_width(Self::LINE_WIDTH);
        context.begin_path();

        context.move_to(390.0, 180.0);
        context.line_to(390.0 + 100.0, 180.0 + 100.0);

        context.move_to(360.0, 190.0);
        context.line_to(360.0 + 100.0, 190.0 + 100.0);

        context.move_to(337.0, 207.0);
        context.line_to(337.0 + 100.0, 207.0 + 100.0);

        context.move_to(320.0, 230.0);
        context.line_to(320.0 + 98.0, 230.0 + 98.0);

        context.move_to(308.0, 262.0);
        context.line_to(308.0 + 100.0, 262.0 + 100.0);

        context.stroke();
    }
}
