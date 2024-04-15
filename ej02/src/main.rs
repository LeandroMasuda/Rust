/*2- Escribir un programa que defina una variable de tipo entero sin signo, y luego imprima su
valor en hexadecimal. */
fn main() {
    let decimal_number = 42;// :X pasa un numero a hexa la X tiene q ser mayuscula 
    let hex_string = format!("{:x}", decimal_number); // Lowercase hexadecimal
    println!("Hexadecimal representation: {}", hex_string);
}
