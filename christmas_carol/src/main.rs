fn main() {

    let lyrics = ["And a partridge in a pear tree.",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
"Twelve drummers drumming,",
    ];

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelveth"];


    println!(
    "On the {} day of Christmas, my true love gave to me,
a partridge in a pear tree
    ", days[0]);

    for day in (1..12) {
        println!("On the {} day of Christmas, my true love gave to me", days[day]);
        for verse in (0..day+1).rev() {
            println!("{}", lyrics[verse]);
        }
        println!("")
    }
    
}
