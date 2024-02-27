fn main() {

    // // create a comment line
    /*
       /* */ this creates a comment block
        where you can make multiple lines a comment
    

     /// triple / makes a library doc for the following item
     //! Generates library doc for the enclosing item

//format!("text") write formatted text to string
format!("test");

//print! works the same as format! but text is printed on console
print!("pipipipopopo");

//println! is the same as print! but a new line is posted
println!("chipi chipi");
println!("chapa chapa");

//eprint! 
eprint!("dubi dubi");

//eprintln!
eprint!("daba daba");
*/

    // let = declaração de variavel
    let variavel: i32 = 10;
    println!("variavel é: {variavel}");
    println!("...Somando variavel + 1...");
    //variavel = variavel + 1;
    if variavel != 10
    {
        println!("wtf? how?");
    }
    else
    {
        println!("variavel agora é: {variavel}");
    }
    
    
    // mut = variavel mutavel, que pode ser modificada posteriormente
    let mut variar: i32 = 20;
    println!("Variar é: {variar}");
    variar = variar + 10;
    println!("Variar agora é: {variar}");

    let x :i32 = 2;
    let z :i32 = 10;

    let soma:i32 = x + z;
    println!("Somando x + z = {soma}");

    let subtracao:i32 = x - z;
    println!("Subtraindo x - z = {subtracao}");

    let multiplicar:i32 = x * z;
    println!("Multiplicando x * z = {multiplicar}");

    let dividir:i32 = z / x;
    println!("Dividindo z / x = {dividir}");

    let restante:i32 = x % z;
    println!("restante de x / z = {restante}");
}
