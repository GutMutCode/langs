mod counter;
mod todos;

use counter::Counter;
use submillisecond::{router, static_router, Application};
use submillisecond_live_view::prelude::LiveViewRouter;
use todos::Todos;

fn main() -> std::io::Result<()> {
    Application::new(router! {
        GET "/" => Counter::handler("views/todos.html", "#app")
        GET "/" => Todos::handler("views/todos.html", "#app")
        "/static" => static_router!("./static")
    })
    .serve("127.0.0.1:3000")
}
