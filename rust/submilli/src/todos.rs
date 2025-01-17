use serde::{Deserialize, Serialize};
use submillisecond::http::Uri;
use submillisecond_live_view::{html, rendered::Rendered, socket::Socket, LiveView, LiveViewEvent};
use uuid::Uuid;

//============================================================================
// NOTE: live view
//============================================================================
#[derive(Debug)]
pub struct Todos {
    filter: Filter,
    todos: Vec<Todo>,
}

// implement the LiveView trait for Todos
impl LiveView for Todos {
    type Events = (Add, Edit, ToggleEdit, SetFilter, ClearCompleted, Remove);

    fn mount(_uri: Uri, _socket: Option<Socket>) -> Self {
        Self {
            filter: Filter::All,
            todos: vec![Todo::default()],
        }
    }

    fn render(&self) -> Rendered {
        let lendered = html! {
            section.todoapp {
                @(self.render_header())

                @if !self.todos.is_empty() {
                    @(self.render_main())
                    @(self.render_footer())
                }
            }
        };

        lendered
    }
}

impl Todos {
    // render the header html
    fn render_header(&self) -> Rendered {
        html! {
            header.header {
                h1 { "todos" }

                form #newtodo
                    method="post"
                    autocapitalzie="off"
                    autocomplete="off"
                    autocorrect="off"
                    spellcheck="false"
                    url="#"
                    @submit=(Add)
                {
                    i {
                        input #newtodo_text .new-todo autofocus name="title" placeholder="What needs to be done?" type="text";
                    }
                    button.hidden type="submit" { "submit" }
                }
            }
        }
    }

    fn render_main(&self) -> Rendered {
        let visible_todos: Vec<_> = match self.filter {
            Filter::All => self.todos.iter().collect(),
            Filter::Active => self.todos.iter().filter(|todo| !todo.completed).collect(),
            Filter::Completed => self.todos.iter().filter(|todo| todo.completed).collect(),
        };

        html! {
            section.main {
                input #toggle-all.toggle-all type="checkbox";
                label for="toggle-all" { "Mark all as complete" }
                ul.todo-list {
                    @for todo in visible_todos {
                        @let classes = match (todo.completed, todo.editing) {
                            (true, true) => "completed editing",
                            (true, false) => "completed",
                            (false, true) => "editing",
                            (false, false) => "",
                        };
                        li class=(classes) {
                            @let id = todo.id.to_string();
                            form
                                method="post"
                                autocapitalzie="off"
                                autocomplete="off"
                                autocorrect="off"
                                spellcheck="false"
                                url="#"
                                @submit=(Edit)
                            {
                                div.view {
                                    input.toggle
                                        type="checkbox"
                                        checked[todo.completed]
                                        :id=(id)
                                        @click=(ToggleEdit);
                                    label :id=(id) @click=(ToggleEdit) {
                                        (todo.title)
                                    }
                                    button.destory :id=(id) type="button" @click=(Remove) {}
                                }
                                input type="hidden" name="id" value=(id);
                                input.edit name="title" value=(todo.title);
                            }
                        }
                    }
                }
            }
        }
    }

    fn render_footer(&self) -> Rendered {
        let remaining_todos = self.todos.iter().filter(|todo| !todo.completed).count();
        let filter_links = [
            ("All", Filter::All),
            ("Active", Filter::Active),
            ("Completed", Filter::Completed),
        ]
        .into_iter()
        .map(|(label, filter)| (label, filter, filter == self.filter));
        html! {
            section.footer {
                span.todo-count {
                    strong { (remaining_todos) }
                    " item(s) left"
                }

                ul.filters {
                    @for (label, filter, selected) in filter_links {
                        li {
                            @let selected_class = if selected { "selected" } else { "" };
                            @let filter_value = serde_json::to_string(&filter).unwrap();
                            a
                                class=(selected_class)
                                href={"#/" (label)}
                                :filter=(filter_value.trim_matches('"'))
                                @click=(SetFilter)
                            {
                                (label)
                            }
                        }
                    }
                }

                @if remaining_todos > 0 {
                    button.clear-completed @click=(ClearCompleted) { "Clear completed" }
                }
            }

            footer.info {
                p { "Double-click to edit a todo"}
                p {
                    "Written by "
                    a href="https://github.com/skoler" { "skoler" }
                }
                p {
                    "Part of "
                    a href="https://github.com/lunatic-solutions/submillisecond-live-view" { "Submillisecond Live View" }
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct Todo {
    id: Uuid,
    title: String,
    completed: bool,
    editing: bool,
}

impl Todo {
    fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            completed: false,
            editing: false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
enum Filter {
    All,
    Active,
    Completed,
}

//============================================================================
// NOTE: Add live view events
//============================================================================
#[derive(Deserialize)]
pub struct Add {
    title: String,
}
impl LiveViewEvent<Add> for Todos {
    fn handle(state: &mut Self, event: Add) {
        state.todos.push(Todo::new(event.title))
    }
}

#[derive(Deserialize)]
pub struct Edit {
    id: Uuid,
    title: String,
}
impl LiveViewEvent<Edit> for Todos {
    fn handle(state: &mut Self, event: Edit) {
        if let Some(todo) = state.todos.iter_mut().find(|todo| todo.id == event.id) {
            todo.title = event.title;
            todo.editing = false;
        }
    }
}

#[derive(Deserialize)]
pub struct ToggleEdit {
    id: Uuid,
    detail: u8,
}
impl LiveViewEvent<ToggleEdit> for Todos {
    fn handle(state: &mut Self, event: ToggleEdit) {
        if event.detail == 2 {
            if let Some(todo) = state.todos.iter_mut().find(|todo| todo.id == event.id) {
                todo.editing = true;
            }
        }
    }
}

#[derive(Deserialize)]
pub struct SetFilter {
    filter: Filter,
}
impl LiveViewEvent<SetFilter> for Todos {
    fn handle(state: &mut Self, event: SetFilter) {
        state.filter = event.filter;
    }
}

#[derive(Deserialize)]
pub struct ClearCompleted {}
impl LiveViewEvent<ClearCompleted> for Todos {
    fn handle(state: &mut Self, _event: ClearCompleted) {
        state.todos.retain(|todo| !todo.completed)
    }
}

#[derive(Deserialize)]
pub struct Remove {
    id: Uuid,
}
impl LiveViewEvent<Remove> for Todos {
    fn handle(state: &mut Self, event: Remove) {
        state.todos.retain(|todo| todo.id != event.id)
    }
}
