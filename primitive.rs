fn main(){
    //Primitive variables
    let logical: bool = true; //Boolean

    let a_float: f64 = 1.64; //Regular annotation
    let an_integer= 5i32; //Suffix annotation

    let default_float = 3.0; //f64
    let default_integer= 7; //i32

    // A type can also be inferred from context 
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    //A mutable variable value can change
    let mut mutable = 12;
    mutable = 32;

    //Varibale can be overwritten with shadowing
    let _mutable = true;
}
