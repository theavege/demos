#![forbid(unsafe_code)]
mod model;
use {
    cairo::{Context, Format, ImageSurface},
    fltk::{
        app,
        button::Button,
        draw,
        enums::{Align, Color, ColorDepth, Event, Font},
        frame::Frame,
        group::Flex,
        image::{RgbImage, SvgImage},
        prelude::*,
        window::Window,
    },
    model::Model,
    std::{cell::RefCell, rc::Rc},
};

const NAME: &str = "FlCairoButton";

#[derive(Debug, Clone)]
struct View {
    window: Window,
    value: Frame,
    inc: Button,
    dec: Button,
}

impl View {
    fn default() -> Self {
        Self {
            window: crate::build_window(),
            value: crate::build_frame(),
            dec: crate::build_button().with_label("@<"),
            inc: crate::build_button().with_label("@>"),
        }
    }
    fn design(&mut self) {
        self.window.begin();
        let mut vbox = Flex::default()
            .with_size(600, 200)
            .center_of_parent()
            .column();
        vbox.end();
        vbox.set_pad(0);
        vbox.set_margin(0);
        {
            let mut hbox = Flex::default();
            hbox.end();
            hbox.add(&self.dec);
            hbox.add(&self.value);
            hbox.add(&self.inc);
            vbox.add(&hbox);
        }
        self.window.end();
        self.window.show();
    }
    fn load(&mut self) {
        self.value.do_callback();
        self.window.do_callback();
    }
    fn update(&mut self, state: Rc<RefCell<Model>>) {
        self.inc.set_callback({
            let state = state.clone();
            let mut view = self.clone();
            move |_| {
                state.borrow_mut().inc();
                view.load();
            }
        });
        self.dec.set_callback({
            let state = state.clone();
            let mut view = self.clone();
            move |_| {
                state.borrow_mut().dec();
                view.load();
            }
        });
        self.value.set_callback({
            let state = state.clone();
            move |frame| {
                frame.set_label(&state.borrow().value());
            }
        });
        self.window.set_callback(move |window| {
            window.set_label(&format!("{} - {NAME}", state.borrow().value()));
            if app::event() == Event::Close {
                state.borrow().save();
                app::quit();
            }
        });
        self.load();
    }
}

fn main() -> Result<(), FltkError> {
    let application = app::App::default();
    let mut view = View::default();
    view.design();
    view.update(Rc::from(RefCell::from(Model::default())));
    application.run()
}

fn build_window() -> Window {
    let mut element = Window::default()
        .with_label(NAME)
        .with_size(640, 360)
        .center_screen();
    element.set_xclass(NAME);
    element.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
    ));
    element.set_color(Color::from_u32(0xfdf6e3));
    element.make_resizable(false);
    element.end();
    element
}

fn build_frame() -> Frame {
    let mut element = Frame::default();
    element.set_label_size(60);
    element
}

fn build_button() -> Button {
    let mut element = Button::default();
    element.super_draw(false);
    element.draw(move |button| {
        draw::draw_rect_fill(
            button.x(),
            button.y(),
            button.w(),
            button.h(),
            Color::from_u32(0xfdf6e3),
        );
        let mut surface = ImageSurface::create(Format::ARgb32, button.w(), button.h())
            .expect("Couldn’t create surface");
        crate::draw_surface(&mut surface, button.w(), button.h());
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
    });
    element
}

fn draw_surface(surface: &mut ImageSurface, w: i32, h: i32) {
    let ctx = Context::new(surface).unwrap();
    ctx.save().unwrap();
    let corner_radius = h as f64 / 10.0;
    let radius = corner_radius / 1.0;
    let degrees = std::f64::consts::PI / 180.0;

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
