use blog_shared::Post;

fn main() {
    let post = Post::new(
        String::from("Post on the server"),
        String::from("Let's Get Rusty!")
    );

    println!("{:?}", post);
}
