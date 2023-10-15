  pub trait Summary {
    fn summarize_author(&self) -> String;
    // fn summarize(&self) -> String;
    // Default implementations
    fn summarize(&self) -> String{
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
  fn summarize_author(&self) -> String {
      format!("The Author of: {}", self.author)
  }
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
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
  // Will use Default implementations
  // fn summarize(&self) -> String {
  //     format!("{}: {}", self.username, self.content)
  // }
}


// Traits as parameters
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify_bound<T: Summary>(item: &T) {
  println!("Notifyng summarize: {}", item.summarize());
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Specifying Multiple Trait Bounds with the + Syntax

// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {


// Clearer Trait Bounds with where Clauses

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

  // fn some_function<T, U>(t: &T, u: &U) -> i32
  // where
  //     T: Display + Clone,
  //     U: Clone + Debug,
  // {}

  // Returning Types that Implement Traits
  fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around 
// how the impl Trait syntax is implemented in the compiler. We’ll cover how to write a
//  function with this behavior in the “Using Trait Objects That Allow for Values of Different Types” 

  // fn returns_summarizable_v2(switch: bool) -> impl Summary {
  //   if switch {
  //       NewsArticle {
  //           headline: String::from(
  //               "Penguins win the Stanley Cup Championship!",
  //           ),
  //           location: String::from("Pittsburgh, PA, USA"),
  //           author: String::from("Iceburgh"),
  //           content: String::from(
  //               "The Pittsburgh Penguins once again are the best \
  //                hockey team in the NHL.",
  //           ),
  //       }
  //   } else {
  //       Tweet {
  //           username: String::from("horse_ebooks"),
  //           content: String::from(
  //               "of course, as you probably already know, people",
  //           ),
  //           reply: false,
  //           retweet: false,
  //       }
  //   }
  // }
    
  use std::fmt::Display;

  struct Pair<T> {
      x: T,
      y: T,
  }
  
  impl<T> Pair<T> {
      fn new(x: T, y: T) -> Self {
          Self { x, y }
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