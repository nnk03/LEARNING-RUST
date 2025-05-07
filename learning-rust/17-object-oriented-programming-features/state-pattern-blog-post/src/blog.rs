#![allow(dead_code)]

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    pub num_approve_calls: u32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            num_approve_calls: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.add_text(&mut self.content, text));
        }

        // self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(&mut self.num_approve_calls));
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject(&mut self.num_approve_calls));
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>, num_approve_calls: &mut u32) -> Box<dyn State>;

    // the below won't work because the compiler has to know the size at compile time
    // fn approve(self: Box<Self>, num_approve_calls: &mut u32) -> Box<dyn State> {
    //     self
    // }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>, num_approve_calls: &mut u32) -> Box<dyn State>;

    fn add_text(self: Box<Self>, content: &mut String, text: &str) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>, _num_approve_calls: &mut u32) -> Box<dyn State> {
        // do not change num_approve_calls while in Draft
        self
    }

    fn reject(self: Box<Self>, _num_approve_calls: &mut u32) -> Box<dyn State> {
        self
    }

    fn add_text(self: Box<Self>, content: &mut String, text: &str) -> Box<dyn State> {
        content.push_str(text);
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, num_approve_calls: &mut u32) -> Box<dyn State> {
        if *num_approve_calls == 0 {
            *num_approve_calls += 1;
            return self;
        }

        if *num_approve_calls == 1 {
            *num_approve_calls += 1;
        }

        // if num_approve_calls == 2, then return Published
        Box::new(Published {})
    }

    fn reject(self: Box<Self>, num_approve_calls: &mut u32) -> Box<dyn State> {
        if *num_approve_calls == 2 {
            *num_approve_calls = 1;
            return self;
        }

        if *num_approve_calls == 1 {
            *num_approve_calls = 0;
            return self;
        }

        Box::new(Draft {})
    }

    fn add_text(self: Box<Self>, _content: &mut String, _text: &str) -> Box<dyn State> {
        // can't add content in PendingReview state
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, _num_approve_calls: &mut u32) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>, _num_approve_calls: &mut u32) -> Box<dyn State> {
        self
    }

    fn add_text(self: Box<Self>, _content: &mut String, _text: &str) -> Box<dyn State> {
        // can't add content in PublishedState
        self
    }
}
