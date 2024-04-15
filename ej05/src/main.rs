/*5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario
ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la
cadena en mayúsculas. */
use std::io::stdin;
fn main() {
    let mut frase:String="Inicio ".to_string();
    let mut conca=String::new();
    println!("Ingresar concatenacion");
    stdin().read_line(&mut conca).expect("Error al leer. ");
    frase+=&conca;
    frase = frase.to_uppercase();
    println!("{}",frase);
}
