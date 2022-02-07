fn main(){
    println!("1+2 = {}", 1+2);
    println!("2-1 = {}", 2-1);

    //Logical
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    //Bit operations
    println!("0011 AND 0101 is {:04b}",0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    //I can use underscore to improve redability
    println!("I can write One million as {}", 1_000_000u32);
}
