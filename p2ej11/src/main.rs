/*11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
por el parámetro factor modificándolo. */

fn multiplicar_valores(arreglo:&mut[i32;6],factor:i32){
    let mut cont=0;
    for i in *arreglo{
        arreglo[cont]=arreglo[cont]*factor;
        cont+=1;
    }
}
#[test]
fn main(){
    let mut a=[4,3,5,6,4,5];
    multiplicar_valores(&mut a,3);
    assert_eq!(a,[12,9,15,18,12,15]);
}