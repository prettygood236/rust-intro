fn main() {
    //* Integer
    //. The signed integers are: i8, i16, i32, i64, i128, and isize. 
    //. The unsigned integers are: u8, u16, u32, u64, u128, and usize.
    let my_number = 100; //. type지정을 안하면 i32로 배정된다.
    let my_other_number: u8 = 50; //. u8 -> 255까지 가능.
    let _third_number = my_number + my_other_number;
    //. 컴파일러가 자동으로 my_number, third_number의 타입을 u8으로 배정한다.
    // println!("{}", third_number);

    //* Char 
    //. char는 4 bytes의 메모리를 차지한다. 
    let _first_letter = 'A'; //. char는 오직 한글자만 가능하다. 
    let _space = ' '; // A space inside ' ' is also a char
    let _other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let _cat_face = '😺'; // Emojis are chars too

    //- casting : simple type change using 'as'
    let first_number: u16 = 8;
    let second_number: u8 = 10;
    let _third_number = first_number + second_number as u16;
    let _my_num = 'a' as u8;
    // println!("Hello, world! My number is {}", my_num); //. a -> (ASCII) 97

    // println!("Size of a char: {}", std::mem::size_of::<char>()); //. 4 bytes
    // println!("Size of string containing 'a': {}", "a".len()); //. .len() gives the size of the string in bytes
    // println!("Size of string containing 'ß': {}", "ß".len());
    // println!("Size of string containing '国': {}", "国".len());
    // println!("Size of string containing '𓅱': {}", "𓅱".len());

    let _slice = "Hello!";
    // println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let _slice2 = "안녕!";
    // println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());

    let _my_numm = 9_______u8; // === let my_numm: u8 = 9; 
    let _other_numm = 1____000___000_____000u64;
    // println!("{}",other_numm);

    //* Float
    //. f32, f64 두가지가 있다.
    let my_nummm = 9.6; // f64
    let other_nummm = 9; // i32
    println!("{}",my_nummm as i32 + other_nummm); // 18
    println!("{}",my_nummm + other_nummm as f64); // 18.6
}
  