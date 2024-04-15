/*10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
del arreglo que son de longitud mayor al parámetro límite.
 */


 fn cantidad_de_cadenas_mayor_a(arreglo:[&str;4],limite:usize)->i32{
    let mut cant:i32=0;
    let mut cont=0;
    let mut ca:i32;
    for i in arreglo{
        if limite < i.len() {
            cant+=1
        }
        }
    cant
 }


#[test]
 fn test_cantidad_de_cadenas_mayor_a(){
    let a=["hola","hlaaa","h","holaaaa"];
    let cant=cantidad_de_cadenas_mayor_a(a,4);
    assert_eq!(2,cant);
 }