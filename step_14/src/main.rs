fn main() {
    let player_1 = String::from("Player 1");
    let player_2 = String::from("Player 2");

    let result = first_turn(&player_1, &player_2);
    println!("The first turn is for: {}", result);

    let tweet_content = String::from("This is a tweet");
    let mut tweet = Tweet::new(&tweet_content);
    println!("Tweet: {}", tweet.content);

    let new_tweet_content = String::from("This is a new tweet");
    let old_content = tweet.replace_content(&new_tweet_content);
    println!("Old content: {}", old_content);

    let content = String::from("This is a content");
    let result = take_and_return_content_1(&content);
    println!("Result: {}", result);

    let content_1 = String::from("This is a content 1");
    let content_2 = String::from("This is a content 2");
    let result = take_and_return_content_2(&content_1, &content_2);
    println!("Content 1: {}", result);
}

// This will get an error because the lifetime of the return value is not known
// fn first_turn(player_1: &str, player_2: &str) -> &str {
//     if rand::random_bool(0.5) {
//         player_1
//     } else {
//         player_2
//     }
// }
// To reslove this, we can use the lifetime annotation
// Lifetime annotations are a way to describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes of the values they refer to.
// The lifetime annotation syntax is a single quote followed by a name, like 'a.
fn first_turn<'a>(player_1: &'a str, player_2: &'a str) -> &'a str {
    if rand::random_bool(0.5) {
        player_1
    } else {
        player_2
    }
}

struct Tweet<'a> {
    content: &'a str
}

impl<'a> Tweet<'a> {
    fn new(content: &'a str) -> Tweet<'a> {
        Tweet {
            content
        }
    }

    // Due to the third rule, the lifetime of self is assigned to all output lifetime parameters.
    // So we can remove the lifetime annotation from the return value.
    // fn replace_content<'b>(&mut self, content: &'b str) -> &'b str {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

// Lifetime elision rules
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.

// Because we have a single input lifetime parameter, the lifetime of the return value is the same as the lifetime of the input parameter. 
// So we can remove the lifetime annotation from the return value.
fn take_and_return_content_1(content: &str) -> &str {
    content
}

// Because we return content_1, the lifetime of the return value is the same as the lifetime of content_1.
fn take_and_return_content_2<'a, 'b>(content_1: &'a str, content_2: &'b str) -> &'a str {
    println!("Content 2: {}", content_2);
    content_1
}