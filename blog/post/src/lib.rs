pub struct Post {
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
        self.content = self.state.as_ref().unwrap().add_text(&self.content, text)
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
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
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text(&self, original_text: &str, _text_to_add: &str) -> String {
        original_text.to_string()
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text(&self ,original_text: &str, text_to_add: &str) -> String {
        format!("{}{}",original_text,text_to_add)
    }
}

struct PendingReview {
    approvals: usize
}

impl PendingReview {
    fn new() -> Self {
        PendingReview {
            approvals: 0
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approvals += 1;
        if self.approvals >= 2 {
            Box::new(Published {})
        } else {
            self
        }

    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}








#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_draft() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        post.add_text("\nAnd a steak!");
        assert_eq!("", post.content());
    }
    #[test]
    fn test_pending_review() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        post.add_text("\nAnd a steak!");
        post.request_review();
        post.add_text("\nAnd dessert!");
        assert_eq!("", post.content()); 
    }
    #[test]
    fn test_published() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        post.add_text("\nAnd a steak!");
        post.request_review();
        post.add_text("\nAnd dessert!");
        post.approve();
        post.approve();
        post.add_text("\nAnd coffee!");
        assert_eq!("I ate a salad for lunch today\nAnd a steak!", post.content());
    }

}