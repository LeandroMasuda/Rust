/*12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de
enteros, y luego imprima la cadena y la suma de los valores en el arreglo */
fn main(){
    let  tupla=("String".to_string(),[2,5,6]);
    println!("Cadena: {}",tupla.0);
    let suma=tupla.1[0]+tupla.1[1]+tupla.1[2];
    println!("La suma del arreglo de la tupla es: {}",suma);
}