use serde::{Deserialize, Serialize};

use submillisecond::{router, static_router, Application, NamedParam};
use submillisecond_live_view::prelude::*;
#[derive(NamedParam)]
pub struct User {
    first_name: String,
    last_name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Counter {
    count: i32,
}

impl LiveView for Counter {
    type Events = (Increment, Decrement);

    fn mount(_uri: Uri, _socket: Option<Socket>) -> Self {
        Counter { count: 0 }
    }

    fn render(&self) -> Rendered {
        html! {
            button @click=(Increment) { "Increment" }
            button @click=(Decrement) { "Decrement" }
            p { "Count is " (self.count)}
        }
    }
}

#[derive(Deserialize)]
pub struct Increment {}

impl LiveViewEvent<Increment> for Counter {
    fn handle(state: &mut Self, _event: Increment) {
        state.count += 1;
    }
}

#[derive(Deserialize)]
pub struct Decrement {}

impl LiveViewEvent<Decrement> for Counter {
    fn handle(state: &mut Self, _event: Decrement) {
        state.count -= 1;
    }
}

// Handlers
//
// Handlesrs are functions which return a response which implements `IntoResponse`.
// They can have any number of arguments, where each argument is an extractor.
pub fn hi(user: User) -> String {
    format!("Hello {} {}!", user.first_name, user.last_name)
}

fn main() -> std::io::Result<()> {
    // defining a router
    Application::new(router! {
        GET "/" => Counter::handler("src/index.html", "#app")
        GET "/hi/:first_name/:last_name" => hi
        "/static" => static_router!("./static")
        // POST "/update_data" => update_age
    })
    .serve("0.0.0.0:3000")
}
