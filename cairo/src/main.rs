use {
    cairo::Context,
    cascade::cascade,
    fltk::{enums::*, frame::Frame, image::SvgImage, prelude::*, *},
};

fn main() -> Result<(), FltkError> {
    let app = app::App::default().with_scheme(app::AppScheme::Base);
    cascade!(window::Window::default()
        .with_label("Demo: Cairo")
        .with_size(260, 260)
        .center_screen();
        ..set_color(Color::White);
        ..make_resizable(true);
        ..set_icon(Some(
            SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
        ));
        ..add(&cascade!(
            cairowidget(5, 5, 100, 100, "Box1");
            ..set_color(Color::Red);
            ..handle(crate::change);
        ));
        ..add(&cascade!(
            cairowidget(80, 80, 100, 100, "Box2");
            ..set_color(Color::Yellow);
            ..handle(crate::change);
        ));
        ..add(&cascade!(
            cairowidget(155, 155, 100, 100, "Box3");
            ..set_color(Color::Green);
            ..handle(crate::change);
        ));
        ..end();
    )
    .show();
    app::cairo::set_autolink_context(true);
    app.run()
}

fn draw_box_with_alpha(rect: &mut Frame) {
    let ctx = unsafe { Context::from_raw_none(fltk::app::cairo::cc() as _) };
    let (r, g, b) = rect.color().to_rgb();
    ctx.save().unwrap();
    ctx.move_to(rect.x() as f64, rect.y() as f64);
    ctx.line_to((rect.x() + rect.w()) as f64, rect.y() as f64);
    ctx.line_to((rect.x() + rect.w()) as f64, (rect.y() + rect.h()) as f64);
    ctx.line_to(rect.x() as f64, (rect.y() + rect.h()) as f64);
    ctx.close_path();
    ctx.set_source_rgba(
        r as f64 / 255.0,
        g as f64 / 255.0,
        b as f64 / 255.0,
        100.0 / 255.0,
    );
    ctx.fill().unwrap();
    ctx.restore().unwrap();
}

pub fn cairowidget(x: i32, y: i32, w: i32, h: i32, label: &str) -> Frame {
    let mut element = Frame::new(x, y, w, h, None).with_label(label);
    element.super_draw_first(false); // required for windows
    element.draw(draw_box_with_alpha);
    element
}

fn change(frame: &mut Frame, event: Event) -> bool {
    match event {
        Event::Released => {
            match frame.color() {
                Color::Red => frame.set_color(Color::DarkRed),
                Color::DarkRed => frame.set_color(Color::Red),
                Color::Yellow => frame.set_color(Color::DarkYellow),
                Color::DarkYellow => frame.set_color(Color::Yellow),
                Color::Green => frame.set_color(Color::DarkGreen),
                Color::DarkGreen => frame.set_color(Color::Green),
                _ => {}
            };
            app::redraw();
            true
        }
        Event::Enter => {
            frame.window().unwrap().set_cursor(Cursor::Hand);
            true
        }
        Event::Leave => {
            frame.window().unwrap().set_cursor(Cursor::Arrow);
            true
        }
        _ => false,
    }
}
