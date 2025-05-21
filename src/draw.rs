use crate::browser;

pub struct Draw;
pub struct DrawLogo;

impl Draw {
    pub fn draw(scale: f64) {
        let context = browser::context().unwrap();

        DrawLogo::draw_diamond(&context, scale);
        DrawLogo::draw_middle_line(&context, scale);
        DrawLogo::draw_circle(&context, scale);
        DrawLogo::draw_h_in_circle(&context, scale);
        DrawLogo::draw_6lines(&context, scale);
        DrawLogo::curved_sparkle(&context, scale);
        DrawLogo::draw_arc(&context, scale);
        DrawLogo::draw_5lines_between_arc(&context, scale);
        DrawLogo::draw_text(&context, scale);
    }
}

impl DrawLogo {
    const LINE_WIDTH: f64 = 12.0;

    // スケーリングのヘルパー関数
    fn scaled(value: f64, scale: f64) -> f64 {
        value * scale
    }

    // 線の太さもスケーリング
    fn scaled_line_width(scale: f64) -> f64 {
        Self::LINE_WIDTH * scale
    }

    pub fn draw_diamond(context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        context.set_line_width(Self::scaled_line_width(scale));
        context.begin_path();
        context.move_to(Self::scaled(300.0, scale), Self::scaled(50.0, scale));
        context.line_to(Self::scaled(50.0, scale), Self::scaled(300.0, scale));
        context.line_to(Self::scaled(300.0, scale), Self::scaled(550.0, scale));
        context.line_to(Self::scaled(550.0, scale), Self::scaled(300.0, scale));
        context.line_to(Self::scaled(300.0, scale), Self::scaled(50.0, scale));
        context.close_path();
        context.stroke();
    }

    pub fn draw_middle_line(context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        context.set_line_width(Self::scaled_line_width(scale));
        context.begin_path();
        context.move_to(Self::scaled(175.0, scale), Self::scaled(175.0, scale));
        context.line_to(Self::scaled(425.0, scale), Self::scaled(425.0, scale));
        context.stroke();
    }

    pub fn draw_circle(context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        context.set_line_width(Self::scaled_line_width(scale));
        context.begin_path();
        context
            .arc(
                Self::scaled(168.0, scale),
                Self::scaled(300.0, scale),
                Self::scaled(78.0, scale),
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        context.stroke();
    }

    pub fn draw_h_in_circle(context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        context.set_line_width(Self::scaled_line_width(scale));
        context.begin_path();
        context.move_to(Self::scaled(125.0, scale), Self::scaled(365.0, scale));
        context.line_to(Self::scaled(125.0, scale), Self::scaled(235.0, scale));
        context.move_to(Self::scaled(125.0, scale), Self::scaled(300.0, scale));
        context.line_to(Self::scaled(205.0, scale), Self::scaled(300.0, scale));
        context.move_to(Self::scaled(205.0, scale), Self::scaled(235.0, scale));
        context.line_to(Self::scaled(205.0, scale), Self::scaled(365.0, scale));
        context.stroke();
    }

    pub fn draw_6lines(context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        let adjust = Self::scaled(127.0, scale);
        let gap = Self::scaled(23.0, scale);

        context.set_line_width(Self::scaled_line_width(scale));
        context.begin_path();
        context.move_to(Self::scaled(405.0, scale), Self::scaled(405.0, scale));
        context.line_to(
            Self::scaled(405.0, scale) - adjust,
            Self::scaled(405.0, scale) + adjust,
        );

        context.move_to(
            Self::scaled(405.0 - 1.0 * gap, scale),
            Self::scaled(405.0 - 1.0 * gap, scale),
        );
        context.line_to(
            Self::scaled(405.0 - 1.0 * gap - adjust, scale),
            Self::scaled(405.0 - 1.0 * gap + adjust, scale),
        );

        context.move_to(
            Self::scaled(405.0 - 2.0 * gap, scale),
            Self::scaled(405.0 - 2.0 * gap, scale),
        );
        context.line_to(
            Self::scaled(405.0 - 2.0 * gap - adjust, scale),
            Self::scaled(405.0 - 2.0 * gap + adjust, scale),
        );

        context.move_to(
            Self::scaled(405.0 - 3.0 * gap, scale),
            Self::scaled(405.0 - 3.0 * gap, scale),
        );
        context.line_to(
            Self::scaled(405.0 - 3.0 * gap - adjust, scale),
            Self::scaled(405.0 - 3.0 * gap + adjust, scale),
        );

        context.move_to(
            Self::scaled(405.0 - 4.0 * gap, scale),
            Self::scaled(405.0 - 4.0 * gap, scale),
        );
        context.line_to(
            Self::scaled(405.0 - 4.0 * gap - adjust, scale),
            Self::scaled(405.0 - 4.0 * gap + adjust, scale),
        );

        context.move_to(
            Self::scaled(405.0 - 5.0 * gap, scale),
            Self::scaled(405.0 - 5.0 * gap, scale),
        );
        context.line_to(
            Self::scaled(405.0 - 5.0 * gap - adjust, scale),
            Self::scaled(405.0 - 5.0 * gap + adjust, scale),
        );

        context.stroke();
    }

    pub fn curved_sparkle(context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        context.set_line_width(Self::scaled_line_width(scale));
        context.set_line_join("bevel");

        context.begin_path();
        context
            .arc(
                Self::scaled(174.0, scale),
                Self::scaled(48.5, scale),
                Self::scaled(125.5, scale),
                std::f64::consts::PI * 0.0,
                std::f64::consts::PI * 0.5,
            )
            .unwrap();
        context
            .arc(
                Self::scaled(174.0, scale),
                Self::scaled(300.5, scale),
                Self::scaled(125.5, scale),
                std::f64::consts::PI * 1.5,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        context
            .arc(
                Self::scaled(425.5, scale),
                Self::scaled(300.5, scale),
                Self::scaled(125.5, scale),
                std::f64::consts::PI * 1.0,
                std::f64::consts::PI * 1.5,
            )
            .unwrap();
        context
            .arc(
                Self::scaled(425.5, scale),
                Self::scaled(48.5, scale),
                Self::scaled(125.5, scale),
                std::f64::consts::PI * 0.5,
                std::f64::consts::PI * 1.0,
            )
            .unwrap();

        context.stroke();
    }

    pub fn draw_arc(context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        context.set_line_width(Self::scaled_line_width(scale));
        context.begin_path();
        context
            .arc(
                Self::scaled(492.0, scale),
                Self::scaled(365.0, scale),
                Self::scaled(85.0, scale),
                std::f64::consts::PI * 0.75,
                std::f64::consts::PI * 1.75,
            )
            .unwrap();
        context.stroke();
    }

    pub fn draw_5lines_between_arc(context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        context.set_line_width(Self::scaled_line_width(scale));
        context.begin_path();

        context.move_to(Self::scaled(390.0, scale), Self::scaled(180.0, scale));
        context.line_to(
            Self::scaled(390.0 + 100.0, scale),
            Self::scaled(180.0 + 100.0, scale),
        );

        context.move_to(Self::scaled(360.0, scale), Self::scaled(190.0, scale));
        context.line_to(
            Self::scaled(360.0 + 100.0, scale),
            Self::scaled(190.0 + 100.0, scale),
        );

        context.move_to(Self::scaled(337.0, scale), Self::scaled(207.0, scale));
        context.line_to(
            Self::scaled(337.0 + 100.0, scale),
            Self::scaled(207.0 + 100.0, scale),
        );

        context.move_to(Self::scaled(320.0, scale), Self::scaled(230.0, scale));
        context.line_to(
            Self::scaled(320.0 + 98.0, scale),
            Self::scaled(230.0 + 98.0, scale),
        );

        context.move_to(Self::scaled(308.0, scale), Self::scaled(262.0, scale));
        context.line_to(
            Self::scaled(308.0 + 100.0, scale),
            Self::scaled(262.0 + 100.0, scale),
        );

        context.stroke();
    }

    pub fn draw_text(context: &web_sys::CanvasRenderingContext2d, scale: f64) {
        context
            .translate(Self::scaled(440.0, scale), Self::scaled(20.0, scale))
            .unwrap();
        context.rotate(std::f64::consts::PI / 4.0).unwrap();
        context.set_text_align("center");
        context.set_font("36px KOMET");
        context
            .fill_text(
                "HATSUBOSHI GAKUEN",
                Self::scaled(100.0, scale),
                Self::scaled(100.0, scale),
            )
            .unwrap();
        context.restore();
    }
}
