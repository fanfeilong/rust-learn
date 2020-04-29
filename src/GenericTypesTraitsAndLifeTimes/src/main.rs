fn main() {
    {
        let number_list  = vec![34,50,25,100,65];
        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("{}", largest);
    }

    {
        let number_list = vec![102,34,6000,89,54,2,43,8];
        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("{}", largest);
    }

    {
        let number_list = vec![102,34,6000,89,54,2,43,8];
        let largest = largest(&number_list);
        println!("{}", largest);
    }

    {
        let i = Point { x: 5, y: 10};
        let f = Point { x: 1.0, y: 4.0};
       
        // let f = Point { x: 1, y: 4.0};

        println!("{}", i.x());
        println!("{}", f.x());


        let f = Point2 { x: 1, y: 4.0};

    }

    {
        // monomorphization at compile time
    }


    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };
    
        println!("1 new tweet: {}", tweet.summarize());

        notify2(&tweet);

        notify(tweet);
        
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: std::cmp::PartialOrd+std::marker::Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct  Point<T> {
    x: T,
    y: T
}

#[derive(Debug)]
struct  Point2<T,U> {
    x: T,
    y: U
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

impl<T>  Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T,U> Point2<T,U> {
    fn mixup<V,W>(self, other: Point2<V,W>) -> Point2<T,W> {
        Point2{
            x: self.x,
            y: other.y
        }
    }
}

pub trait Summary2 {
    fn summarize_author(&self) -> String;
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        // default implementations
        // Default implementations can call other methods in the same trait, 
        // even if those other methods don’t have a default implementation. 
        format!("(Read more..), {}", self.summarize_author())
    }
}


#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String{
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String{
        format!("{}", self.username)
    }
    // fn summarize(&self) -> String{
    //     format!("{}, by {} {}", self.username, self.content, self.reply)
    // }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
pub fn notify2<T: Summary>(item: &T){
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item: impl Summary, item2: impl Summary) {
    // add code here
}

pub fn notify4<T: Summary>(item: T, item2: T) {
    // add code here
}

pub fn notify5(item: impl Summary+Summary2) {
    // add code here
}

pub fn notify6<T:Summary+Summary2>(item: T) {
    // add code here
}

pub fn notify7<T>(item: T) -> i32 where T: Summary+Summary2 {
    // add code here
    1
}

pub fn notify8<T>(item: T) -> T where T: Summary+Summary2 {
    // add code here
    item
}

pub fn notify9<T>(item: T) -> impl Summary where T: Summary+Summary2 {
    // add code here
    item
}

use std::fmt::Display;
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,y
        }
    }
}

impl<T:Display+PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x>=self.y {
            println!("The largest member is x={}", self.x);
        }else {
            println!("The largest member is y={}", self.y);
        }
    }
}

// blanket implementations
// conditionally implement a trait for any type that implements another trait.
// impl<T: Display> ToString for T {
//     // add code here
// }