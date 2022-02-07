fn reverse(pair: (i32, bool)) -> (bool, i32){//Tuples as arguments and return values
    let (integer, boolean) = pair;

    (boolean, integer)
}

//the following struct is for activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32); 

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64, 'a', true);
    
    //Values can be extracted by the indexing 
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    //tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    //Tuples are printable
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

    //But long tuples are not printable
    
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("But the reverse pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    //     // from a literal surrounded by parentheses
    println!("Just one element tuple :{:?}", (2u32,));
    println!("But this is not a tuple :{:?}", (2u32));

    //tuples can be destructured to create bindings
    let tuple = (1, 4.5, "a", true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    
    let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("Matrix: {:?}", matrix);
}
