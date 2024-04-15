/*4- Definir la función llamada cantidad_impares que recibe como parámetro un arreglo de
números enteros y retorna la cantidad de números impares. */


fn cantidad_impares(arreglo:[i32;4])->i32{
    let mut imp:i32=0;
    for i in arreglo{
        if i%2!=0 {imp+=1}
    }
    imp
}


#[test]
fn test_cantidad_impares(){
    let arreglo=[4,8,6,2];
    let cant=cantidad_impares(arreglo);
    assert_eq!(0,cant);
}