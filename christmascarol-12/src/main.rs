const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "A partridge in a pear tree.",
    "Two turtle doves, ",
    "Three French hens, ",
    "Four calling birds, ",
    "Five golden rings, ",
    "Six geese a-laying, ",
    "Seven swans a-swimming, ",
    "Eight maids a-milking, ",
    "Nine ladies dancing, ",
    "Ten lords a-leaping, ",
    "Eleven pipers piping, ",
    "Twelve drummers drumming, ",
];

fn main() {
    for day in 1..=12 {
        println!("On the {} day of Christmas,", ORDINALS[day - 1]);
        println!("my true love sent to me");

        for index in (0..day).rev() {
            println!("{}", GIFTS[index]);
        }
        println!();
    }
}
