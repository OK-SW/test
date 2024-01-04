fn main() {
    let ly = [
        ("first", "A partridge in a peer tree"),
        ("second", "Two turtle doves"),
        ("third", "Three French hens"),
        ("fourth", "Five golden rings"),
        ("sixth", "Six geese a-laying"),
        ("seventh", "Seven swans a-swimming"),
        ("eighth", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelfth", "Twelve drummers drumming")
    ];

    for i in (1..12){
        println!("On the {} day of Christmas,", ly[i - 1].0);
        println!("my true love gave to me");

        for j in (1..i + 1).rev(){
            println!("{}",ly[j - 1].1);
        }
        println!();
    }
}
