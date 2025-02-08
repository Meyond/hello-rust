// trait类似于其他语言的接口，但是Rust的trait可以包含默认实现。

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

pub struct Book {
  pub title: String,
  pub author: String,
  pub year: u32,
}

// 定义 Summary 特征，包含一个方法 summarize 默认实现返回一段提示信息
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Tip...)")
  }
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

impl Summary for Book {
  fn summarize(&self) -> String {
    format!("{} by {} ({})", self.title, self.author, self.year)
  }
}

// 定义一个函数，接受实现了 Summary 特征的任何类型的引用，并调用其 summarize 方法
fn notify<T: Summary>(item: &T) {
  println!("News! {}", item.summarize());
}

fn main() {
  let article = NewsArticle {
    headline: String::from("New Study on Climate Change"),
    location: String::from("New York"),
    author: String::from("John Doe"),
    content: String::from("A new study reveals the latest findings on climate change."),
  };

  let book = Book {
    title: String::from("The Rust Programming Language"),
    author: String::from("Steve Klabnik and Carol Nichols"),
    year: 2018,
  };

  notify(&article);
  notify(&book);
}

// 测试代码
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_news_article_summary() {
    let article = NewsArticle {
      headline: String::from("New Study on Climate Change"),
      location: String::from("New York"),
      author: String::from("John Doe"),
      content: String::from("A new study reveals the latest findings on climate change."),
    };
    assert_eq!(
      article.summarize(),
      "New Study on Climate Change, by John Doe (New York)"
    );
  }

  #[test]
  fn test_book_summary() {
    let book = Book {
      title: String::from("The Rust Programming Language"),
      author: String::from("Steve Klabnik and Carol Nichols"),
      year: 2018,
    };
    assert_eq!(
      book.summarize(),
      "The Rust Programming Language by Steve Klabnik and Carol Nichols (2018)"
    );
  }
}
