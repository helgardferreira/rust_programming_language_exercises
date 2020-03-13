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
        self.content = self
            .state
            .as_ref()
            .unwrap()
            .add_text(&self.content, text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    fn add_text(&self, original_text: &str, _new_text: &str) -> String
    {
        original_text.to_string()
    }
    /*
    If we attempted to give request_review and approve default implementations we
    would get the following compile error:
    */
    /*
    fn request_review(self: Box<Self>)
                      -> Box<dyn State> {
        self
    }
    */
    /*
    error[E0277]: the size for values of type `Self` cannot be known at
                  compilation time
      --> src\lib.rs:67:9
       |
    66 |                       -> Box<dyn State> {
       |                                        - help: consider further
                                                  restricting `Self`: `where Self:
                                                  std::marker::Sized`
    67 |         self
       |         ^^^^ doesn't have a size known at compile-time
       |
       = help: the trait `std::marker::Sized` is not implemented for `Self`
       = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
       = note: required for the cast to the object type `dyn State`
    */
    // this method consumes the previous self and returns a dynamic Boxed version
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // default implementation for content to return an empty string slice
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn add_text(&self, original_text: &str, new_text: &str) -> String {
        format!("{}{}", original_text, new_text)
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { num_approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview { num_approvals: u8 }

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.num_approvals += 1;
        if self.num_approvals >= 2 {
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
    fn draft_returns_empty_string() {
        let mut draft = Post::new();

        draft.add_text("Hello world!");

        assert_eq!("", draft.content())
    }

    #[test]
    fn draft_is_published() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        // drafts require at least two approvals
        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn pending_review_is_rejected() {
        let mut post = Post::new();

        post.add_text("Hello world!");
        post.request_review();
        post.reject();

        assert_eq!(post.content(), "");

        post.approve();

        assert_eq!(post.content(), "");
    }

    #[test]
    fn only_allow_draft_amend() {
        let mut post = Post::new();

        post.add_text("Hello world!");

        assert_eq!("", post.content());

        post.request_review();
        post.approve();
        post.approve();

        assert_eq!("Hello world!", post.content());

        post.add_text(" My name is John Doe.");

        assert_eq!("Hello world!", post.content());
    }

    #[test]
    fn post_is_rejected_then_amended() {
        let mut post = Post::new();

        post.add_text("Hello...");

        post.request_review();
        post.approve();
        post.reject();

        assert_eq!("", post.content());

        post.add_text(" World!");

        post.request_review();
        post.approve();
        post.approve();

        assert_eq!("Hello... World!", post.content());
    }
}

/*
Trade-offs of the State Pattern

We've shown that Rust is capable of implementing the object-oriented state pattern
to encapsulate the different kinds of behavior a post should have in each state.
The methods on Post know nothing about the various behaviors. The way we organized
the code, we have to look in only one place to know the different ways a published
post can behave: the implementation of the State trait on the Published struct.

If we were to create an alternative implementation that didn't use the state
pattern, we might instead use match expressions in the methods on Post or even in
the main code that checks the state of the post and changes behavior in those
places. That would mean we would have to look in several places to understand all
the implications of a post being in the published state! This would only increase
the more states we added: each of those match expressions would need another arm.

With the state pattern, the Post methods and the places we use Post don't need
match expressions, and to add a new state, we would only need to add a new struct
and implement the trait methods on that one struct.

The implementation using the state pattern is easy to extend to add more
functionality.

One downside of the state pattern is that, because the states implement the
transitions between states, some of the states are coupled to each other. If we
add another state between PendingReview and Published, such as Scheduled, we would
have to change the code in PendingReview to transition to Scheduled instead. It
would be less work if PendingReview didn't need to change with the addition of a
new state, but that would mean switching to another design pattern.

Another downside is that we've duplicated some logic. To eliminate some of the
duplication, we might try to make default implementations for the request_review
and approve methods on the State trait that return self; however, this would
violate object safety, because the trait doesn't know what the concrete self will
be exactly. We want to be able to use State as a trait object, so we need its
methods to be object safe.

Other duplication includes the similar implementations of the request_review and
approve methods on Post. Both methods delegate to the implementation of the same
method on the value in the state field of Option and set the new value of the
state field to the result. If we had a lot of methods on Post that followed this
pattern, we might consider defining a macro to eliminate the repetition.

By implementing the state pattern exactly as it's defined for object-oriented
languages, we're not taking as full advantage of Rust's strengths as we could.
*/
