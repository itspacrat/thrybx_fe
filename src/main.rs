use {
    core::f32,
    // iced stuff to visually show the implementation and make it interactive
    iced::{
        Element, Settings, alignment, run, widget::{button, column, row, slider, text}, window
    },
    // audio sources and players to use the thrybx structures
    rodio::{OutputStream, buffer::SamplesBuffer, source::Source},
    // std for more f32 constants, file IO and stream types
    std::{f32::consts::PI, fs::File, io::BufReader, sync::Arc},
    // implementation lib (the soul of this app)
    thrybx::*,
};
/*
! ICED SETUP
*/
pub struct AppState {
    counter1: i64,
    counter2: i64,
}
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Unhide,
}

fn update(appState: &mut AppState, message: Message) {
    match message {
        Message::Increment => *counter += 1,
        Message::Decrement => *counter -= 1,
        Message::Unhide => {
            println!("unhidden")
        }
    }
}

fn view(app_state: &AppState) -> Element<Message> {
    let mut grid: Element<_> = column![
        row![
            button(text("-")).on_press(Message::Decrement),
            text(counter),
            button(text("+")).on_press(Message::Increment)
        ].align_y(alignment::Vertical::Center)
        .spacing(50),
        row![
            button(text("-")).on_press(Message::Decrement),
            text(counter),
            button(text("+")).on_press(Message::Increment)
        ].align_y(alignment::Vertical::Center)
        .spacing(30),
        row![button(text("press")).on_press(Message::Decrement)].spacing(40)
    ].spacing(20).align_x(alignment::Horizontal::Center)
    .into();
    grid // send 'er out
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
