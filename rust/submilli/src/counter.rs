use serde::{Deserialize, Serialize};
use submillisecond_live_view::prelude::{html, LiveView, LiveViewEvent, Rendered, Socket, Uri};

// counter has a state
#[derive(Clone, Serialize, Deserialize)]
pub struct Counter {
    count: i32,
}

impl LiveView for Counter {
    type Events = (Increment, Decrement);
    fn mount(_uri: Uri, _socket: Option<Socket>) -> Self {
        Self { count: 0 }
    }

    fn render(&self) -> Rendered {
        html! {
            button class="bg-blue-400" @click=(Increment) {"Increment"}
            button @click=(Decrement) {"Decrement"}

            p class="p_tag" {"Count is " (self.count)}

            @if self.count >= 5 {
                p {"Count is high!"}
            }
        }
    }
}

// Events
#[derive(Serialize, Deserialize)]
pub struct Increment {}

impl LiveViewEvent<Increment> for Counter {
    fn handle(state: &mut Self, _event: Increment) {
        state.count += 1
    }
}

#[derive(Serialize, Deserialize)]
pub struct Decrement {}

impl LiveViewEvent<Decrement> for Counter {
    fn handle(state: &mut Self, _event: Decrement) {
        state.count -= 1
    }
}
