/*7-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
números mayores al límite que tiene el arreglo. */

fn cantidad_de_mayores(arreglo:[i32;6],limite:i32)->i32{
    let mut cant=0;
    for i in arreglo{
        if i>limite {
            cant+=1;
        }
    }
    cant
}


#[test]
fn test_cantidad_de_mayores(){
    let arreglo=[4,54,43,23,6,34];
    let limite=20;
    let cant=cantidad_de_mayores(arreglo,limite);
    assert_eq!(4,cant);
}