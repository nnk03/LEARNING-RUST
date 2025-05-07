#![allow(dead_code)]

mod blog;

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review(); // Draft to PendingReview
    assert_eq!("", post.content());

    post.add_text("Hello"); // can't add when in pending review state
    assert_eq!("", post.content());

    post.reject(); // PendingReview to Draft
    assert_eq!("", post.content());

    post.add_text(", Hello"); // Back to draft, so can add, but won't show it yet
    assert_eq!("", post.content());

    post.request_review(); // Draft to PendingReview
    assert_eq!("", post.content());

    post.approve(); // PendingReview to PendingReview, but num_approve_calls increased
    assert_eq!("", post.content());
    println!("count = {}", post.num_approve_calls);

    post.reject();
    assert_eq!("", post.content());
    println!("count = {}", post.num_approve_calls);

    post.approve();
    println!("count = {}", post.num_approve_calls);
    post.approve(); // after 2 approve calls, PendingReview to Published
    println!("count = {}", post.num_approve_calls);
    assert_eq!("I ate a salad for lunch today, Hello", post.content());
}
