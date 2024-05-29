pub struct Post {
    // Box: reference for compile
    // trait object : to have dynamic type (polymorphism) - state determine at runtime
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().is_draft() {
            self.content.push_str(text);
        }
        // Option<&Box<dyn State>>
    }

    // method is the same, no matter state is.
    pub fn request_review(&mut self) {
        // Rust doesn't let us to have unpoputated field so we should use take() method
        if let Some(s) = self.state.take() {
            // run state's method (encapsulation)
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    // pub fn request_review(&mut self) {}
    // pub fn approve(&mut self) {}
    pub fn content(&self) -> &str {
        // self.state.unwrap().content(self)
        self.state.as_ref().unwrap().content(self)
    }
}

#[allow(unused_variables)]
trait State {
    // this is shared to states -> separate per each state
    // self: methods are valid only for state
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn is_draft(&self) -> bool {
        false
    }
}

struct Draft {}
struct PendingReview {}
struct PendingFinReview {}
struct Published {}

impl State for Draft {
    // this method only works with valid parameter
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // take ownership
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn is_draft(&self) -> bool {
        true
    }
}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Box::new(Published {})
        Box::new(PendingFinReview {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}
impl State for PendingFinReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        // Box::new(Draft {})
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
