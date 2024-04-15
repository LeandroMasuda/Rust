/*3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario
ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones
and y or. Se deben imprimir ambos resultados.*/
use std::io::stdin;
fn main() {
    let bol=true;
    let  mut bol1;
    let mut ingreso=String::new();
    println !("Ingresar boolean");
    stdin().read_line(&mut ingreso).expect("Error");
    bol1=ingreso.trim().parse().expect("Error");
    println!("Imprime con AND");
    println!("{}",bol1&&bol);
    println!("Imprime con OR");
    println!("{}",bol1||bol);
    
}
