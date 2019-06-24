pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// longer form:
// pub fn notify<T: Summary>(item: T) {
//    println!("Breaking news! {}", item.summarize());
// }

// To accept two items of any type that implement Summary:
// pub fn notify(item1: impl Summary, item2: impl Summary) {}

// To accept two items of the same types that implement Summary:
// pub fn notify<T: Summary>(item1: T, item2: T) {}

// Multiple trait bounds with + Syntax:
// pub fn notify(item: impl Summary + Display)

// The + syntax is also valid with trait bounds on generic types:
// pub fn notify<T: Summary + Display>(item: T)