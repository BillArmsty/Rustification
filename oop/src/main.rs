use blog::Post;

fn main() {
    //Blog crate 
    let mut post = Post::new();

    post.add_text("I'll wait for her tonight");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I'll wait for her tonight", post.content());
}
