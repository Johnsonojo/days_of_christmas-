fn main() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves, and",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    println!("The Twelve Days of Christmas");
    println!();

    for day in 1..=12 {
        println!("On the {} day of Christmas,", days[day - 1]);
        println!("My true love gave to me:");

        for gift_day in (0..day).rev() {
            if gift_day == 0 && day > 1 {
                print!("And ");
            }
            println!("{}", gifts[gift_day]);
        }

        println!();
    }
}
