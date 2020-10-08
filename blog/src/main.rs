use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Ich habe heute Mittag einen Salat gegessen");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Ich habe heute Mittag einen Salat gegessen", post.content());
}
