fn main() {
    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
    // taking advantage of the repetition in the song.

    let mut bank = 2;
    let mut bank2 = 2;

    //Make an array for each line of the song
    //Ordered so that each index corresponds to the respective verse
    //You cannot use a for loop with negative integers, so the actual
    //song has to start from 1 instead of 0
    let christmascarol = [
        "Fuck Rust",
        "And a Partridge in a Pear Tree.",
        "Two Turtle Doves,",
        "Three French Hens,",
        "Four calling Birds,",
        "Five Golden Rings,",
        "Six Geese a Laying,",
        "Seven Swans a Swimming,",
        "Eight Maids a Milking,",
        "Nine Ladies Dancing,",
        "Ten Lords a Leaping,",
        "Eleven Pipers Piping,",
        "12 Drummers Drumming,",
    ];
   
   
    println!("On the 1st day of Christmas my true love gave to me:");
    println!("A Partridge in a Pear Tree.");
   
   
    while bank != christmascarol.len() {
        println!("On the {bank}{} day of Christmas my true love gave to me:", if bank == 2 { "nd"} else if bank == 3 {"rd"} else {"th"});
        //Using "bank" to repeatedly print the most recent lyrics and the ones before it.
        while bank2 != 0 {
            println!("{y}", y = christmascarol[bank2]);
            bank2 -= 1;
        }
        //Add counter to the base of what needs to be called now.
        //similarly to the fibonacci sequence exercise
        bank += 1;
        bank2 = bank;
    }
}