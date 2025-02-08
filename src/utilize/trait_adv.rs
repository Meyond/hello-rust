// trait bound 和 impl Trait 语法是等价的
fn notify(item: &impl Summary) {}

fn notify<T: Summary>(item: &T) {}

// 通过 where 简化 trait bound
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{
}

// 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}
