/*
The structs no longer have the state field because we're moving the encoding of the
state to the types of the structs. The Post struct will represent a published post,
and it has a content method that returns the content.

This is far more idiomatic (in my opinion), and it is definitely the more "rusty"
way of doing things.
*/
pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content
        }
    }
}

/*
The request_review and approve methods take ownership of self, thus consuming the
DraftPost and PendingReviewPost instances and transforming them into a
PendingReviewPost and a published Post, respectively. This way, we won't have any
lingering DraftPost instances after we've called request_review on them, and so
forth. The PendingReviewPost struct doesn't have a content method defined on it, so
attempting to read its content results in a compiler error, as with DraftPost.
Because the only way to get a published Post instance that does have a content
method defined is to call the approve method on a PendingReviewPost, and the only
way to get a PendingReviewPost is to call the request_review method on a DraftPost,
we've now encoded the blog post workflow into the type system.
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;

    /*
    N.B. The following function's output and the type_name crate may change in the
    future! Only use the below functionality for diagnostics. In this case (for
    unit testing) it is completely fine.
    */
    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn draft_is_published_and_content_is_accessible() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn post_is_chainable() {
        let mut post = Post::new();
        post.add_text("Hello World!");

        assert_eq!("Hello World!", post.request_review().approve().content());
    }

    #[test]
    fn pending_review_is_rejected() {
        let mut post = Post::new();

        post.add_text("Hello world!");
        let post = post.request_review();
        let post = post.reject();

        assert_eq!("encoding_states::DraftPost", type_of(post))
    }
}
