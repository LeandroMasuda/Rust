/*4- Escribir un programa que defina una tupla que contenga una cadena, un número entero
con signo y un valor booleano, y luego imprima cada valor de la tupla
 */
fn main(){
    let mut tupla:(String,u64,bool)=("ej4".to_string(),25,true);
    println!("{:?}",tupla);
}