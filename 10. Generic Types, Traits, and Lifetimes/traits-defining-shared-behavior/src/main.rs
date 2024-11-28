use traits_defining_shared_behavior::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, you probably already know, people"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The pittsburgh Penguins once again are the best \
            hockey team in the NHL"
        )
    };

    println!("1 new tweet: {}", tweet.summarize());
    // println!("New article available! {}", article.summarize());

}
