use core::fmt::Display;
use core::fmt::Formatter;


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people",),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    notify(&article);
    notify(&tweet);

    let pair = Pair {
        x: 10,
        y: 10,
    };
    pair.new(10, 20).cmp_display();
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} {}", self.headline, self.author, self.location)
    // }
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

pub fn notify<T>(item: &T)
where
    T: Summary + Display
{
    println!("Breaking news! {}", item.summarize());
}

impl Display for Tweet {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}
impl Display for NewsArticle {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}

fn return_summarizable(_switch: bool) -> impl Summary {
    NewsArticle {
        headline: String::from(
                      "Penguins win the Stanley Cup Championship!",
                      ),
                      location: String::from("Pittsburgh, PA, USA"),
                      author: String::from("Iceburgh"),
                      content: String::from(
                          "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
                 ),
    }
    // if switch {
    //     NewsArticle {
    //         headline: String::from(
    //             "Penguins win the Stanley Cup Championship!",
    //         ),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //         ),
    //     }
    // } else {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from(
    //             "of course, as you probably already know, people",
    //         ),
    //         reply: false,
    //         retweet: false,
    //     }
    // }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(&self, x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
