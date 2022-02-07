fn main(){
    //in general, the {} will be replaced with any arguments
    println!("{} days", 31);

        // Without a suffix, 31 becomes an i32. You can change what type 31 is
        //     // by providing a suffix. The number 31i64 for example has the type i64.
        //
        //         // There are various optional patterns this works with. Positional
        //             // arguments can be used.

    println!("{0}, This is {1}. {1}, this is {0}", "Alice", "Bob");
    

    //Also i can name arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the brown fox",
             verb="jump over");

    // Special formatting can be specified by an ':'
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
}
