/*
3- Definir la función llamada suma_pares que recibe como parámetro un arreglo de
números enteros y retorna la suma de los números pares. */

fn suma_pares(arreglo:[i32;4])->i32{
    let mut suma=0;
    for num in arreglo{
        if num%2==0{
            suma+=num;
        }       
    }
    suma

}
#[test]
fn test_suma_pares(){
    let arreglo=[1,5,6,2];
    let suma=suma_pares(arreglo);
    assert_eq!(8,suma);
}