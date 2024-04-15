/*1-Definir la función llamada es_par que recibe como parámetro un número entero y retorna
true si el número es par, false caso contrario. */

fn main(){
    let num=10;
    let ok=es_par(num);
    println!("Es par? {} ",ok);
}

fn es_par(data:i32)->bool{
    return if data%2==0{true}else{false};
}

#[test]
fn test_es_par(){
    let num=10;
    let ok=es_par(num);
    assert!(ok);
}