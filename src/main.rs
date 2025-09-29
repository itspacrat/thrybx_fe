use {
    core::f32,
    // iced stuff to visually show the implementation and make it interactive
    iced::{
        Alignment, Element, Length::Fill, Settings, alignment, run, widget::{button, column, container, row, slider, text}, window
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

#[derive(Default)]
pub struct AppState {
    counter1: i64,
    counter2: i64,
}
#[derive(Debug, Clone)]
enum Message {
    Increment1,
    Decrement1,
    Increment2,
    Decrement2,
    DecrementBoth,
    Unhide,
}

fn update(app_state: &mut AppState, message: Message) {
    match message {
        Message::Increment1 => app_state.counter1 += 1,
        Message::Increment2 => app_state.counter2 += 1,
        Message::Decrement1 => app_state.counter1 -= 1,
        Message::Decrement2 => app_state.counter2 -= 1,
        Message::DecrementBoth => {
            app_state.counter1 -= 1;
            app_state.counter2 -= 1;
        }
        Message::Unhide => {
            println!("unhidden")
        }
    }
}

fn view(app_state: &AppState) -> Element<Message> {
    let mut grid: Element<_> = container(
        row![
            text(app_state.counter1),
            text(app_state.counter2)
            ].spacing(20)
    ).width(Fill).height(Fill)
    .align_x(Alignment::Center).align_y(Alignment::Center)
    .into();
    // container(column![
    //     row![
    //         button(text("-")).on_press(Message::Decrement1),
    //         text(app_state.counter1),
    //         button(text("+")).on_press(Message::Increment1)
    //     ].align_y(alignment::Vertical::Center)
    //     .spacing(80).width(Fill),
    //     row![
    //         button(text("-")).on_press(Message::Decrement2),
    //         text(app_state.counter2),
    //         button(text("+")).on_press(Message::Increment2)
    //     ].align_y(alignment::Vertical::Center)
    //     .spacing(30).width(Fill),
    //     row![
    //         button(text("subtract from both")).on_press(Message::DecrementBoth)
    //         ].spacing(40)
    // ].spacing(20).align_x(alignment::Horizontal::Center).height(Fill))
    // .into();
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
