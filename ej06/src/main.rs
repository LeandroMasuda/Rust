/*6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al
usuario ingresar un número entero por teclado para sumarse con la variable definida. El
programa debe imprimir el valor del número elevado al cuadrado. */
use std::io::stdin;
fn main(){
    let mut numero:u8=3;
    let mut sumando=String::new();
    println !("Ingresar el numero para sumar a 3");
    stdin().read_line(&mut sumando).expect("Error");
    let numero2:u8 =  sumando.trim().parse().expect("Error");
    let mut impri =numero +numero2;
    impri=(((impri*impri)*impri)*impri);// .pow(2) seria para elevar al cuadrado
    println!("La suma es: {}",impri);
}