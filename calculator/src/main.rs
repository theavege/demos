#![forbid(unsafe_code)]

mod model;

use {
    fltk::{
        app,
        button::Button,
        enums::{Align, CallbackTrigger, Color, Cursor, Event, Font, FrameType, Key, Shortcut},
        frame::Frame,
        group::Flex,
        image::SvgImage,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::*,
        text::{TextBuffer, TextDisplay, WrapMode},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    model::Model,
    std::{cell::RefCell, rc::Rc},
};

const NAME: &str = "FlCalculator";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const EQUAL: &str = "=";
const COLORS: [[Color; 6]; 2] = [
    [
        Color::from_hex(0xfdf6e3),
        Color::from_hex(0x586e75),
        Color::from_hex(0xb58900),
        Color::from_hex(0xeee8d5),
        Color::from_hex(0xcb4b16),
        Color::from_hex(0xdc322f),
    ],
    [
        Color::from_hex(0x002b36),
        Color::from_hex(0x93a1a1),
        Color::from_hex(0x268bd2),
        Color::from_hex(0x073642),
        Color::from_hex(0x6c71c4),
        Color::from_hex(0xd33682),
    ],
];

#[derive(Clone)]
struct View {
    window: Window,
    menu: MenuButton,
    prev: Frame,
    operation: Frame,
    current: Frame,
    output: TextDisplay,
    buttons: Vec<Button>,
}

impl View {
    fn default() -> Self {
        Self {
            window: crate::build_window(),
            menu: crate::build_menu(),
            prev: crate::build_output("Previous"),
            operation: crate::build_output("Operation"),
            current: crate::build_output("Current"),
            output: crate::build_display("Output"),
            buttons: Vec::new(),
        }
    }
    fn design(&mut self) {
        self.window.begin();
        let mut vbox = Flex::default_fill().column();
        {
            vbox.add(&self.output);
            let mut hbox = Flex::default_fill();
            {
                hbox.add(&self.operation);
                let mut vbox = Flex::default_fill().column();
                {
                    vbox.add(&self.prev);
                    vbox.add(&self.current);
                }
                vbox.end();
                vbox.set_pad(0);
            }
            hbox.end();
            hbox.set_pad(0);
            hbox.set_margin(0);
            hbox.fixed(&hbox.child(0).unwrap(), HEIGHT);
            vbox.fixed(&hbox, 60);
            let mut buttons = Flex::default_fill().column();
            for line in [
                ["CE", "C", "%", "/"],
                ["7", "8", "9", "x"],
                ["4", "5", "6", "-"],
                ["1", "2", "3", "+"],
                ["0", ".", "@<-", crate::EQUAL],
            ] {
                let mut hbox = Flex::default_fill();
                for label in line {
                    self.buttons.push(crate::build_button(label));
                }
                hbox.end();
                hbox.set_pad(PAD);
                hbox.set_margin(0);
            }
            buttons.end();
            buttons.set_pad(PAD);
            buttons.set_margin(0);
            vbox.fixed(&buttons, 425);
        }
        vbox.end();
        vbox.set_margin(PAD);
        vbox.set_pad(PAD);
        self.window.end();
        self.window.show();
    }
    fn theme(&mut self, theme: usize) {
        for button in &mut self.buttons {
            match button.label().as_str() {
                "C" | "x" | "/" | "+" | "-" | "%" => {
                    button.set_color(crate::COLORS[theme][2]);
                    button.set_label_color(crate::COLORS[theme][0]);
                }
                "CE" => {
                    button.set_color(crate::COLORS[theme][4]);
                    button.set_label_color(crate::COLORS[theme][0]);
                }
                crate::EQUAL => {
                    button.set_color(crate::COLORS[theme][5]);
                    button.set_label_color(crate::COLORS[theme][0]);
                }
                _ => {
                    button.set_color(crate::COLORS[theme][3]);
                    button.set_label_color(crate::COLORS[theme][1]);
                }
            };
        }
        self.load();
    }
    fn load(&mut self) {
        self.prev.do_callback();
        self.operation.do_callback();
        self.current.do_callback();
        self.output.do_callback();
        self.window.do_callback();
    }
    fn update(&mut self, state: Rc<RefCell<Model>>) {
        self.output.set_callback({
            let state = state.clone();
            move |display| {
                display
                    .buffer()
                    .unwrap()
                    .set_text(&state.borrow().output.clone());
                display.scroll(
                    display.buffer().unwrap().text().split_whitespace().count() as i32,
                    0,
                );
                display.set_color(crate::COLORS[state.borrow().theme as usize][0]);
                display.set_text_color(crate::COLORS[state.borrow().theme as usize][1]);
            }
        });
        self.prev.set_callback({
            let state = state.clone();
            move |frame| {
                frame.set_label(&state.borrow().prev.to_string());
                frame.set_color(crate::COLORS[state.borrow().theme as usize][0]);
                frame.set_label_color(crate::COLORS[state.borrow().theme as usize][1]);
            }
        });
        self.operation.set_callback({
            let state = state.clone();
            move |frame| {
                frame.set_label(&state.borrow().operation.to_string());
                frame.set_color(crate::COLORS[state.borrow().theme as usize][0]);
                frame.set_label_color(crate::COLORS[state.borrow().theme as usize][1]);
            }
        });
        self.current.set_callback({
            let state = state.clone();
            move |frame| {
                frame.set_label(&state.borrow().current.to_string());
                frame.set_color(crate::COLORS[state.borrow().theme as usize][0]);
                frame.set_label_color(crate::COLORS[state.borrow().theme as usize][1]);
            }
        });
        let view = self.clone();
        for button in &mut self.buttons {
            let state = state.clone();
            let mut view = view.clone();
            button.set_callback(move |button| {
                let value = button.label();
                state.borrow_mut().click(&value);
                view.load();
            });
        }
        let mut view = self.clone();
        let item: i32 = self
            .menu
            .add("&Night mode\t", Shortcut::None, MenuFlag::Toggle, {
                let state = state.clone();
                move |_| {
                    state.borrow_mut().theme();
                    view.theme(state.borrow().theme as usize);
                }
            });
        match state.borrow().theme {
            true => self.menu.at(item).unwrap().set(),
            false => self.menu.at(item).unwrap().clear(),
        };
        self.window.handle({
            let menu = self.menu.clone();
            move |window, event| match event {
                Event::Push => match app::event_mouse_button() {
                    app::MouseButton::Right => {
                        menu.popup();
                        true
                    }
                    _ => false,
                },
                Event::Enter => {
                    window.set_cursor(Cursor::Hand);
                    true
                }
                Event::Leave => {
                    window.set_cursor(Cursor::Arrow);
                    true
                }
                _ => false,
            }
        });
        self.window.set_callback({
            let state = state.clone();
            move |window| {
                window.set_color(crate::COLORS[state.borrow().theme as usize][0]);
                window.redraw();
                if app::event() == Event::Close {
                    state.borrow().save();
                    app::quit();
                }
            }
        });
        self.theme(state.borrow().theme as usize);
    }
}

fn main() -> Result<(), FltkError> {
    let app = app::App::default();
    let mut view = View::default();
    view.design();
    view.update(Rc::from(RefCell::from(Model::default())));
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app.run()
}

fn build_window() -> Window {
    let mut element = Window::default()
        .with_size(360, 640)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(false);
    element.set_xclass(NAME);
    element.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
    ));
    element.end();
    element
}

fn build_button(title: &str) -> Button {
    let mut element = Button::default().with_label(title);
    element.set_label_font(Font::CourierBold);
    element.set_label_size(HEIGHT);
    element.set_frame(FrameType::OFlatFrame);
    match title {
        "@<-" => element.set_shortcut(Shortcut::None | Key::BackSpace),
        "CE" => element.set_shortcut(Shortcut::None | Key::Delete),
        crate::EQUAL => element.set_shortcut(Shortcut::None | Key::Enter),
        "x" => element.set_shortcut(Shortcut::None | '*'),
        _ => element.set_shortcut(Shortcut::None | title.chars().next().unwrap()),
    }
    element
}

fn build_display(tooltip: &str) -> TextDisplay {
    let mut element = TextDisplay::default();
    element.set_text_size(HEIGHT - 5);
    element.set_tooltip(tooltip);
    element.set_scrollbar_size(3);
    element.set_frame(FrameType::FlatBox);
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_buffer(TextBuffer::default());
    element.set_trigger(CallbackTrigger::Changed);
    element.set_text_font(Font::CourierBold);
    element
}

fn build_output(tooltip: &str) -> Frame {
    let mut element = Frame::default().with_align(Align::Right | Align::Inside);
    element.set_label_size(HEIGHT);
    element.set_tooltip(tooltip);
    element.set_frame(FrameType::FlatBox);
    element.set_label_font(Font::CourierBold);
    element
}

fn build_menu() -> MenuButton {
    let mut element = MenuButton::default().with_type(MenuButtonType::Popup3);
    element.set_frame(FrameType::FlatBox);
    element
}
