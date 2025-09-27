use {
    core::f32,
    // iced stuff to visually show the implementation and make it interactive
    iced::{
        run,
        widget::{button, column, row, slider, text},
        window, Element, Settings,
    },
    // audio sources and players to use the thrybx structures
    rodio::{buffer::SamplesBuffer, source::Source, OutputStream},
    // std for more f32 constants, file IO and stream types
    std::{f32::consts::PI, fs::File, io::BufReader, sync::Arc},
    // implementation lib (the soul of this app)
    thrybx::*,
};
/*
! ICED SETUP
*/
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Unhide,
    SliderChanged,
}

fn changed_slider(m: Message) {}

fn update(counter: &mut i64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
        Message::Decrement => *counter -= 1,
        Message::Unhide => {println!("unhidden")},
        Message::SliderChanged => {}
    }
}

fn view(counter: &i64) -> Element<Message> {
    let mut grid: Element<_> = column![
        row![
            button(text("-")).on_press(Message::Decrement),
            text(counter),
            button(text("+")).on_press(Message::Increment)
        ]
        .spacing(50),
        row![
            button(text("-")).on_press(Message::Decrement),
            text(counter),
            button(text("+")).on_press(Message::Increment)
        ]
        .spacing(30),
        //row![slider(0..=100,50,|m: Message| -> changed_slider(m))]
    ]
    .into();

    row![button(text("press")).on_press(Message::Unhide)].into()
}

fn main() {
    // set up states

    // load iced window + handle result
    let _app_result = run("counter app", update, view);
    match _app_result {
        Ok(r) => {
            println!("app ran ok!")
        }
        Err(e) => {
            /* do things with error message e */
            println!("app failed with error: {e}")
        }
    }
}
