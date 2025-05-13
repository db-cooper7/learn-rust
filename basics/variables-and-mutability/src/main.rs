fn main() {
    /*
        Rust uses variable bindings to associate names with values. By default, all variables are __immutable__
        which means I cannot reasign a new value after they have been initialized.

        Rust also supports: mutable variables; constants; global static variables;
    
        # let: immutable variables 

            let x = 10;
            x += 1: // Error: cannot assign twice to immutable variable

            x is bound to the value 10
            its type is __inferred__ as i32 (default integer type)
            immutable: attempting to reassing x will cause a compile-time error


        # let mut: mutable variables

            to allow reasignment, use mut

            let mut y: i32 = 2;
            y += 2; // OK

        # const: constants

            must be annotated with a type
            are always __immutable__
            must be assigned __compile-time constant values__
            can be declared in any scope, including global
            are inlined at compile-time, like macros in C

            const MAX_USERS: u32 = 69;

            cannot be the result of a function call or runtime expression

        # static: global variables

            static defines a variable with 'static lifetime' i.e., it lives as long as the program is running

            static SERVER_NAME: &str = "localhost";

            static is used for read-only global data;

            must have a fixed memory location i.e., not dynamically allocated or moved

        # static mut: global mutable variables (__unsafe__)

            rust allows global mutable state using 'static mut', but it is always __unsafe__ to access
            the compiler cannot enforce safe concurrent access or prevent race conditions at compile time

            static mut COUNTER: u32 = 0;

            fn main() {
                unsafe {
                    COUNTER += 1;
                    println!("Counter: {}", COUNTER);
                }
            }

            
            static mut is a low-level feature and it should be avoided unless absolutely necessary (e.g., in embedded or FFI scenarios)
            access must be __wrapped__ in an unsafe block
            there are no automatic protections against data races or undefined behavior

            
            if I need global mutable state safely, I could use interior mutability patterns like __static__ with __Mutex<T>__ or __AtomicUsize__

                use std::sync::Mutex;
                use lazy_static::lazy_static;

                lazy_static! {
                    static ref COUNTER: Mutex<u32> = Mutex::new(0);
                }

                fn main() {
                    let mut count = COUNTER.lock().unwrap();
                    *count += 1;
                }

            this uses lazy_static to safely initialize a global mutex

            # shadowing

            rust allows shadowing: redeclaring a variable with the same name
            this creates a new binding that overrides the previousone
            let x = 1;
            let x = 2; // Shadows the previous 'x'
            let x = x + 2; // Now x is 4
            let x = "Hello, world"; // Now its a string

            useful for transforming values while keeping the same variable name, without needing the mut

            # SUMMARY

            keyword         mutable?        scope           requires type?      notes
            let             no              local           optional            immutable by default
            let mut         yes             local           optional            opt-in mutability
            const           no              global/local    yes                 must be compile-time value
            static          no              global          yes                 read-only static data
            static mut      yes             global          yes                 requires unsafe, not concurrency-safe
    */

    // Immutable variable
    let a = 42;

    // Mutable variable
    let mut b = 10;
    b += 10;

    // Constant 
    const NEPER: f64 = 2.7182818285;

    // Global static
    static GREETING: &str = "Hello there!";

    // Unsafe access to a mutable static
    static mut ERROR_COUNT: u32 = 0;  

    println!("{}", a);
    println!("{}", b);
    println!("{}", NEPER);
    println!("{}", GREETING);
    println!("unsafe {{\n\t ERROR_COUNT += 1;\n\t println!(\"{{}}\", ERROR_COUNT);\n}}");
}

