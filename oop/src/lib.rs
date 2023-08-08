pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            //The state field is private, and the only way to interact with it is through the methods we’ve defined.
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        //The add_text method takes a mutable reference to self so we can change the Post instance that we’ve called add_text on.
        self.content.push_str(text);
    }

    //Implementing the add_text method to add text to a post’s content
    pub fn content(&self) -> &str {
        //The content method takes an immutable reference to self because we only want to read the Post instance, not write to it.
        //It also returns a reference to content data, not owned data, because we don’t want to take ownership of the content data from Post instance.
        self.state.as_ref().unwrap().content(self)
    }

    //Adding a placeholder implementation for the content method on Post that always returns an empty string slice
    pub fn content(&self) -> &str {
        ""
    }

    //Request review method
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    //Approve method
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    //The request_review method on Draft will return a new PendingReview instance.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    //The approve method on Draft will return itself.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    //The request_review method on PendingReview will return itself in this case, because this is the state where we want the post to stay in.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //The approve method on PendingReview will return a new Published instance.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    //The request_review method on Published will return itself in this case, because this is the state where we want the post to stay in.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //The approve method on Published will return itself.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
