use aggregator::{Summary, NewsArticle,  Tweet, notify};

fn main() {
    println!("Hello, world!");
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}