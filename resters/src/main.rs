#![forbid(unsafe_code)]
use {
    fltk::{
        app, dialog,
        enums::{CallbackTrigger, Color, Event, Font, FrameType},
        frame::Frame,
        group::Flex,
        image::SvgImage,
        menu::Choice,
        misc::{InputChoice, Progress},
        prelude::*,
        text::{StyleTableEntry, TextBuffer, TextDisplay, WrapMode},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    json_tools::{Buffer, BufferType, Lexer, Span, TokenType},
    std::thread,
    ureq::{Error, Response},
};

const PAD: i32 = 10;
const HEIGHT: i32 = 3 * PAD;
const WIDTH: i32 = 3 * HEIGHT;

#[derive(Clone)]
struct Widget {
    buffer: TextBuffer,
    choice: Choice,
    input: InputChoice,
    text: TextDisplay,
    status: Progress,
}

impl Widget {
    fn view() -> Self {
        let mut window = crate::window();
        let mut page = Flex::default_fill().column();

        let mut header = Flex::default();
        header.fixed(&Frame::default(), WIDTH);
        let choice = crate::choice();
        header.fixed(&choice, WIDTH);
        header.fixed(&Frame::default(), WIDTH);
        let input = crate::input();
        let status = crate::progress().with_label("Status");
        header.fixed(&status, WIDTH);
        header.end();
        header.set_pad(PAD);
        page.fixed(&header, HEIGHT);

        let hero = Flex::default();
        let buffer = TextBuffer::default();
        let text = crate::text(buffer.clone());
        hero.end();

        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.set_frame(FrameType::FlatBox);
        window.end();
        window.show();
        let mut component = Self {
            buffer,
            choice,
            input,
            text,
            status,
        };
        let mut clone = component.clone();
        component.input.set_callback(move |_| clone.update());
        let mut clone = component.clone();
        component
            .input
            .input()
            .set_callback(move |_| clone.update());
        component
    }
    fn update(&mut self) {
        self.status.set_label("");
        self.text.buffer().unwrap().set_text("");
        self.buffer.set_text("");
        let proto = "https://";
        let path = match self.input.value().unwrap().starts_with(proto) {
            true => self.input.value().unwrap(),
            false => String::from(proto) + &self.input.value().unwrap(),
        };
        let req = match self.choice.value() {
            0 => ureq::get(&path),
            1 => ureq::post(&path),
            _ => unreachable!(),
        };
        let handler = thread::spawn(move || -> Result<Response, Error> { req.call() });
        while !handler.is_finished() {
            app::wait();
            app::sleep(0.02);
            self.status.do_callback();
        }
        if let Ok(req) = handler.join() {
            match req {
                Ok(response) => {
                    if let Ok(json) = response.into_json::<serde_json::Value>() {
                        let json: String = serde_json::to_string_pretty(&json).unwrap();
                        self.text.buffer().unwrap().set_text(&json);
                        self.fill_style_buffer(&json);
                        self.status.set_label("200 OK");
                        self.status.set_label_color(Color::Yellow);
                    } else {
                        dialog::message_default("Error parsing json");
                    }
                }
                Err(Error::Status(code, response)) => {
                    self.status
                        .set_label(&format!("{} {}", code, response.status_text()));
                    self.status.set_label_color(Color::Red);
                }
                Err(e) => {
                    dialog::message_default(&e.to_string());
                }
            }
            self.status.set_value(0f64);
        };
    }
    fn fill_style_buffer(&mut self, s: &str) {
        let mut buffer = vec![b'A'; s.len()];
        for token in Lexer::new(s.bytes(), BufferType::Span) {
            use TokenType::*;
            let c = match token.kind {
                CurlyOpen | CurlyClose | BracketOpen | BracketClose | Colon | Comma | Invalid => {
                    'A'
                }
                String => 'B',
                BooleanTrue | BooleanFalse | Null => 'C',
                Number => 'D',
            };
            if let Buffer::Span(Span { first, end }) = token.buf {
                let start = first as _;
                let last = end as _;
                buffer[start..last].copy_from_slice(c.to_string().repeat(last - start).as_bytes());
            }
        }
        self.buffer.set_text(&String::from_utf8_lossy(&buffer));
    }
}

fn main() -> Result<(), FltkError> {
    Widget::view();
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app::App::default().run()
}

fn window() -> Window {
    const WIDTH: i32 = 640;
    const HEIGHT: i32 = 360;
    const NAME: &str = "FlResters";
    let mut element = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label(NAME)
        .center_screen();
    element.size_range(WIDTH, HEIGHT, 0, 0);
    element.make_resizable(true);
    element.set_xclass(NAME);
    element.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
    ));
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            app::quit();
        }
    });
    element
}

fn text(buffer: TextBuffer) -> TextDisplay {
    let styles: Vec<StyleTableEntry> = [0xdc322f, 0x268bd2, 0x859900]
        .into_iter()
        .map(|color| StyleTableEntry {
            color: Color::from_hex(color),
            font: Font::Courier,
            size: 16,
        })
        .collect();
    let mut element = TextDisplay::default();
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_buffer(TextBuffer::default());
    element.set_color(Color::from_hex(0x002b36));
    element.set_highlight_data(buffer, styles);
    element
}

fn choice() -> Choice {
    let mut element = Choice::default().with_label("Method: ");
    element.add_choice("GET|POST");
    element.set_value(0);
    element
}

fn input() -> InputChoice {
    let mut element = InputChoice::default().with_label("URL: ");
    for item in ["users", "posts", "albums", "todos", "comments", "posts"] {
        element.add(&(format!(r#"https:\/\/jsonplaceholder.typicode.com\/{item}"#)));
    }
    element.add(r#"https:\/\/lingva.thedaviddelta.com\/api\/v1\/languages"#);
    element.add(r#"https:\/\/lingva.thedaviddelta.com\/api\/v1\/en\/de\/mother"#);
    element.add(r#"https:\/\/ipinfo.io\/json"#);
    element.input().set_trigger(CallbackTrigger::EnterKeyAlways);
    element.set_value_index(0);
    element
}

fn progress() -> Progress {
    const MAX: u8 = 120;
    let mut element = Progress::default();
    element.set_maximum((MAX / 4 * 3) as f64);
    element.set_value(element.minimum());
    element.set_callback(move |progress| {
        progress.set_value(if progress.value() == (MAX - 1) as f64 {
            progress.minimum()
        } else {
            progress.value() + 1f64
        })
    });
    element
}
