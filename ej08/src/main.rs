/*8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el
número de veces que un caracter específico ingresado por el usuario aparece en la cadena.
Se debe imprimir el resultado. */

fn main(){
    let cadena ="caaonstante".to_string();
    //let cadena =String::from("Otra forma de hacer una cadena");
    //cadena.
    
    println !(" aparece {}",cadena.matches("a").count());//El matches verifica cuantas veces aparece la letra en los () y el count cuenta
    //println !(" aparece {}",cadena.split("a").count()-1);
    // cada vez q aparece "a" cuenta
}
