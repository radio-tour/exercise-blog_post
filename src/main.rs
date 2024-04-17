use exercise_blog_post::*;

fn main() {
  let mut post = Post::new();

  post.add_text("Hello World!");

  let post = post.request_review();

  let post = post.approve();

  assert_eq!("Hello World!", post.content());
}
