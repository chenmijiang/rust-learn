pub fn main() {
    println!("Chapter 10: Generic Types, Traits, and Lifetimes");

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {result}");

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let result = largest(&number_list);
    // println!("The largest number is {result}");

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {result}");

    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: 'c' };

    // let p3 = p1.mixup(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    demo_trait();
}

// Traits are similar to interfaces in other languages.
// They define shared behavior that types can implement.

// Example trait definition
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    // optional to implement since there's a default
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn demo_trait() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
    };

    println!("New article available! {}", article.summarize());
}

// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
