fn main() {
    //content from rustbook Chapter 3:

    // 3.1 Variables and Immutability    
    let x = 5;
    println!("The value of x is: {x}");

    // this wont compile cuz the variable x is immutable, meaning you cant change this value
    //x = 6;
    //println!("The value of x is: {x}");

    let mut x = 5;
    println!("The value of x is: {x}");

    // this compile cuz the variable x is mutable, meaning you can change its value
    x = 6;
    println!("The value of x is: {x}");

    //CONSTANTs are Immutable by default, and cant be mutable
    //Constants are declared by ( const ) and not with ( let )
    //Constants also *must* be declared
    // constants may be set only to a constant expression, not the result of a value that could only be computed at runtime

    //this wont work
    // const A;

    //this works
    const B:i8 = 1;

    println!("B is: {B}");

    //SHADOWING
    // You can declare variables with the same name, but the code will read the last one and gets its value, using until the last one gets overshadows

    //binding y =5
    let y = 5;
    //overwriting y from 5 to 6
    let y = y + 1;
    {
        //using the last known y value (6) and overwriting with 6 * 2 (12)
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    //reading the last value y has on the program (6) 
    println!("The value of y in the outer scope is {y}");


    // 3.2 Data Types
    // Scalar types represent a single value : Integers Float Booleans Characters
    // Integer can be Signed (i) or unsigned (u)
    // i-type means the number needs to have a sign with it like (-20 20)
    // u-type is for numbers that don't need sign, meaning only positive numbers are accepted
    /*
            8-bit	    i8	    u8
            16-bit	    i16	    u16
            32-bit	    i32	    u32
            64-bit	    i64	    u64
            128-bit	    i128	u128
            arch	    isize	usize
    */

    let pepega: i32 = 123;
    let pepego: i128 = 50000000000;

    // floats = numbers with decimal points

    let skibidi :f64 = 3.45;
    
    
    // numeric operations
    //math stuff
    let ui: i64 = 1;
    let juzi: i64 = 1;

    let cun: i64 = ui + juzi;
    let cun: i64 = ui - juzi;
    let cun: i64 = ui * juzi;
    let cun: i64 = ui / juzi;
    let cun: i64 = ui % juzi;

    //boolean type data
    //booleans return only True or False stats

    let offline = false;
    let is_online: bool = true;

    //Character type
    //Char type is the most primitive alphabetic type
    //examples

    let c = 'z';
    let z:char = 'Z';
    let heart_eyed_cat = '😻';

    // ***** compound Types
    
    //Tuple
    //variable that can hold together a number of values with variety of types

    let tup = ('x',"cunt",69.420,69);
    let (x,y,z,a) = tup;
    println!("{z}");

    let funny_number = tup.3;

    //Arrays

    //Like tuple an array can store multiple values, but they have to be from the same type

    let cue = [1,2,3,4,5,6];

    //arrrays are good to use when you know the number of elements will not need to change
    //like using with months
    //declaring arrays

    let xdi: [i32; 5] = [1,2,3,4,5];
 
    //**************** 3.3 Functions

    new_function();
    a_new_function(1, 3.2);
    
    returning(7);

    notreturning(1);

}


//* FUNCTIONSSSSSSSSS */

//this is a function without paramters
fn new_function()
{
    println!("Another function!!!");
}

//this is a function with parameters
fn a_new_function(a:i32,b:f64)
{
    println!("a is: {} and b is: {}", a, b);
}

//about return 
//this function will return 
fn returning(x: i32) -> i32
{
    x + 1
}

//this wont work cuz its not returning right
fn notreturning(x: i32) -> i32
{
    //not returning cuz Statements don't evaluate to a value, meaning it doesn't return a value, contradicting the function -> i32 return statement
    x+1;
    x
}