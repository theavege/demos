#![forbid(unsafe_code)]

use {
    cascade::cascade,
    chrono::prelude::*,
    fltk::{prelude::*, *},
    fltk_calendar::calendar::Calendar,
};

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Plastic);
    cascade!(
        window::Window::default().with_size(400, 300).center_screen();
        ..make_resizable(true);
        ..set_label("Demo: Calendar");
        ..add(&cascade!(
            button::Button::default().with_size(80, 40).center_of_parent();
            ..set_label("Click");
            ..set_callback(move |_| {
                if let Some(date) = Calendar::default().get_date() {
                    println!("{:?}", date.year());
                    println!("{:?}", date.month());
                    println!("{:?}", date.day());
                }
            });
        ));
        ..end();
    )
    .show();
    app.run().unwrap();
}
