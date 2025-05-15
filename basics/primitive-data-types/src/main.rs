/*
    # Primitive Data Types

    Rust, like C, provides a set of built-in primitive types that represent the most fundamental
    data representations in the language. These include:

    - numbers
    - booleans
    - characters
    - unit
    - function pointers
    - raw pointers

    Primitive types are:

    - copyable (except str, [T], and references unless explicitly stated)
    - stored on the stack (in the majority of cases)
    - passed by value by default

    # Signed Integers:

    Type    |   Size (bits) |   Range
    i8          8               -128 to 127
    i16         16              -32,768 to 32,767
    i32         32              -2,147,483,648 to 2,147,483,647
    i64         64              +/- 9.2e^18
    i128        128             very large range
    isize       arch-dependent  same size as a pointer (32/64-bit)

    let a: i32 = -100;
    let b: isize = 42;

    # Unsigned Integers:

    Type    |   Size (bits) |   Range
    u8          8               0 to 255
    u16         16              0 to 65,535
    u32         32              0 to 4,294,967,295
    u64         64              o to 18.4e^18
    u128        128             very large positive values
    usize       arch-dependent  size of pointer in memory


    let x: u8 = 255;
    let z: usize = 1024;

    # Floating-point Numbers:

    Floating-point numbers follow the IEEE 754 standard

    Type    |   Size    |   Precision
    f32         32          ~6 decimals
    f64         64          ~15 decimals (default)

    let temp_c: f32 = 23.5;
    let distance_km = 42.195; // f64 by default

    # Boolean

    The bool type has only two values: true and false

    let in_use: bool = true;
    let is_expired: bool = false;

    Used in control flow and logical expressions

    # Character

    The char type represents a Unicode Scalar Value. Unlike C's char, rust's is a 32-bit value

    let a: char = 'A';
    let b = 'π';
    let rocket = '🚀';

    Each char can represent any valid UTF-32 encoded

    # Unit Type ()

    The unit type represents the abscence of a value (like NULL in C), it is used:

    - As the return type of a function that returns nothing (like void functions in C)
    - In tuple structs with no fields

    fn do_nothing() {
        // implicitly returns ()
    }

    The only value of type () is the unit value ()

    # Function pointers

    Rust supports pointers as primitive types
    This is mostly used in high-level scenarios like function callbacks

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let f: fn(i32, i32) -> i32 = add;
    let resut = f(2,3);

    Function pointer types use fn(...) -> ... syntax

    # Raw pointers

    Raw pointers are unsafe references used in FFI, low-level memory access, or unsafe code

    let x = 5;
    let ptr: *const i32 = &x;
    let mut y = 10;
    let mut_ptr: *mut i32 = &mut y;

    *const T: raw immutable pointer
    *mut T: raw mutable pointer

    Raw pointer dereferencing requires an unsafe block and does not obey borrow rules

    # Slices and Strings (Special Primitives)

    Slices[T]:

    - Dynamically-sized view into a sequence of elements. Usually accessed via references

    let arr = [1,2,3];
    let slice: &[i32] = &arr[1..3]; // slice contains [2, 3], from index 1 up to (but not including) 3

    String slices: str

    UTF-8 text, dynamicaly sized, usually used as &str

    let name: &str = 'John Motherfucking Doe"

    These are technically DSTs (Dynamically Sized Types) and not primitive in the strictest sense,
    but are built-in and fundamental.

    # Type Inference and Default Types

    When type is omitted:

    - integer defaults to i32
    - float defaults to f64

    let a = 10; // i32
    let b = 3.14 // f64

    it is possible to overide using:

    let a: u8 = 10;
    let b: f32 = 3.14;

*/

fn do_nothing() {

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x = 5;
    let ptr: *const i32 = &x;
    let mut y = 12.345;
    let mut_ptr: *mut f64 = &mut y;
    let rocket: char = '🚀';
    let _f: fn(i32, i32) -> i32 = add;
    let result = add(2,3);
    
    println!("Calling function do_nothing... {:?}", do_nothing());
    println!("The value of x = {}", x);
    println!("The address stored in ptr = {:p}", ptr);
    println!("The value of y = {}", y);
    println!("The address stored in mut_ptr = {:p}", mut_ptr);
    println!("The RB19 is a {}", rocket);
    println!("The result = {}", result);

}
