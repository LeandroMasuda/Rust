/*2- Definir la función llamada es_primo que recibe un número entero positivo mayor a 1 y
    retorna true si es primo, false caso contrario
*/

fn main(){
    let num=1;
    let ok=es_primo(num);
    println!("{num} Es primo {ok}")
}

fn es_primo(data:i32)->bool{
    let mut divi=0;
    for i in 1..=data{
        if data%i==0{divi+=1}
        if divi>2{
            break
        }
    }
    divi<2
}
#[test]
fn test_es_primo(){
    let num=1;
    let ok=es_primo(num);
    assert!(ok);
}