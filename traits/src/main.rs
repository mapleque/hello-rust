pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Display {
    fn display(&self) -> String {
        format!("Show details...")
    }
}

impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("{}", self.content)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking new! {}", item.summarize());
}

pub fn read<T>(item: &T)
where
    T: Summary + Display,
{
    println!("{}\n----\n{}", item.summarize(), item.display());
}

pub fn publish() -> impl Summary {
    Tweet {
        username: String::from("cookie"),
        content: String::from("hello"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let t = publish();
    notify(&t);

    println!("****");

    let a = NewsArticle {
        headline: String::from("A good show!"),
        location: String::from("Beijing"),
        author: String::from("betty"),
        content: String::from("cookie's show is very good!"),
    };
    read(&a);
}
