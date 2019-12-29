pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let new_article = NewArticle {
        headline: String::from("タイトル"),
        location: String::from("日本"),
        author: String::from("山田太郎"),
        content: String::from("本文"),
    };
    println!("{}", new_article.summarize());
}
