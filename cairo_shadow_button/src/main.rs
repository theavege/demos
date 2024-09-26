#![forbid(unsafe_code)]
mod model;
use {
    cairo::{Context, Format, ImageSurface},
    cascade::cascade,
    fltk::{
        app,
        button::Button,
        draw,
        enums::{Align, Color, ColorDepth, Event, Font},
        frame::Frame,
        group::Flex,
        image::RgbImage,
        prelude::*,
        window::Window,
    },
    std::{cell::RefCell, rc::Rc},
};

enum Message {
    Update = 41,
}

impl Message {
    const fn event(self) -> Event {
        Event::from_i32(self as i32)
    }
}

fn main() -> Result<(), FltkError> {
    let state = Rc::new(RefCell::new(model::Model::default()));
    const UPDATE: Event = Message::Update.event();
    let app = app::App::default();
    const NAME: &str = "Demo: Cairo";
    cascade!(
        Window::default().with_label(NAME).with_size(640, 360).center_screen();
        ..set_xclass(NAME);
        ..set_color(Color::White);
        ..make_resizable(true);
        ..set_callback(move |_| {
            if app::event() == Event::Close {
                app::quit();
            }
        });
        ..add(&cascade!(
            Flex::default().with_size(300, 200).center_of_parent().column();
            ..add(&cascade!(
                Frame::default();
                ..handle(glib::clone!(#[strong] state, move |frame, event| {
                    if event == UPDATE {
                        frame.set_label(&state.borrow().value().to_string());
                        return true;
                    }
                    false
                }));
                ..handle_event(UPDATE);
            ));
            ..add(&cascade!(
                Flex::default();
                ..add(&cascade!(
                    Button::default().with_label("@#<");
                    ..super_draw(false);
                    ..draw(add_cairo);
                    ..set_callback(glib::clone!(#[strong] state, move |_|{
                        state.borrow_mut().dec();
                        app::handle_main(UPDATE).unwrap();
                    }));
                ));
                ..add(&cascade!(
                    Button::default().with_label("@#>");
                    ..super_draw(false);
                    ..draw(add_cairo);
                    ..set_callback(glib::clone!(#[strong] state, move |_|{
                        state.borrow_mut().inc();
                        app::handle_main(UPDATE).unwrap();
                    }));
                ));
                ..end();
            ));
            ..end();
        ));
        ..end();
    )
    .show();
    app.run()
}

fn add_cairo(button: &mut Button) {
    draw::draw_rect_fill(button.x(), button.y(), button.w(), button.h(), Color::White);
    let mut surface = ImageSurface::create(Format::ARgb32, button.w(), button.h())
        .expect("Couldn’t create surface");
    draw_surface(&mut surface, button.w(), button.h());
    if !button.value() {
        cairo_blur::blur_image_surface(&mut surface, 20);
    }
    surface
        .with_data(|s| {
            let mut img = RgbImage::new(s, button.w(), button.h(), ColorDepth::Rgba8).unwrap();
            img.draw(button.x(), button.y(), button.w(), button.h());
        })
        .unwrap();
    draw::set_draw_color(Color::Black);
    draw::set_font(Font::Helvetica, app::font_size());
    if button.value() {
        draw::draw_rbox(
            button.x() + 1,
            button.y() + 1,
            button.w() - 4,
            button.h() - 4,
            15,
            true,
            Color::White,
        );
        draw::draw_text2(
            &button.label(),
            button.x() + 1,
            button.y() + 1,
            button.w() - 4,
            button.h() - 4,
            Align::Center,
        );
    } else {
        draw::draw_rbox(
            button.x() + 1,
            button.y() + 1,
            button.w() - 6,
            button.h() - 6,
            15,
            true,
            Color::White,
        );
        draw::draw_text2(
            &button.label(),
            button.x() + 1,
            button.y() + 1,
            button.w() - 6,
            button.h() - 6,
            Align::Center,
        );
    }
}

fn draw_surface(surface: &mut ImageSurface, w: i32, h: i32) {
    let corner_radius = h as f64 / 10.0;
    let radius = corner_radius / 1.0;
    let degrees = std::f64::consts::PI / 180.0;
    let ctx = Context::new(surface).unwrap();
    ctx.save().unwrap();
    ctx.new_sub_path();
    ctx.arc(w as f64 - radius, radius, radius, -90. * degrees, 0.0);
    ctx.arc(
        w as f64 - radius,
        h as f64 - radius,
        radius,
        0.0,
        90. * degrees,
    );
    ctx.arc(
        radius,
        h as f64 - radius,
        radius,
        90. * degrees,
        180. * degrees,
    );
    ctx.arc(radius, radius, radius, 180. * degrees, 270. * degrees);
    ctx.close_path();
    ctx.set_source_rgba(150.0 / 255.0, 150.0 / 255.0, 150.0 / 255.0, 40.0 / 255.0);
    ctx.set_line_width(4.);
    ctx.fill().unwrap();
    ctx.restore().unwrap();
}
