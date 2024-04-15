/*11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario
ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena
ingresada por el usuario se encuentra en el arreglo.
 */
use std::io::stdin;
fn main(){
    let arreglo:[&str; 5]=["primero","segundo","tercero","cuarto","quinto"];
    let mut palabra=String::new();
    println!("Ingresar palabra a buscar");
    stdin().read_line(&mut palabra).expect("Error");
    let mut b:bool=true; 
    println !("Esta en el arreglo la palabra {} {}", palabra,arreglo.contains(&palabra.trim()));

    /* 
    let mut i=0;
    while i<5 && b{
        if arreglo[i]==palabra{
            b=false;
        }
        i+=1;
    }
    
    
    //Forma sin estructura de control
    if arreglo[0]==palabra{
        b=false
    }
    if arreglo[1]==palabra{
        b=false
    }
    if arreglo[2]==palabra{
        b=false
    }
    if arreglo[3]==palabra{
        b=false
    }
    if arreglo[04]==palabra{
        b=false
    }
    if b{
        println!("No se encuentra ");
    }
    else{
        println!("Se encontro ");
    }
    */

}