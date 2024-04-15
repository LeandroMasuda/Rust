/*9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
función retorna la cantidad de números del arreglo que están entre el rango de los
parámetros inferior y superior inclusive. */

fn cantidad_en_rango(arreglo:[i32;4],inferior:i32,superior:i32)->i32{
    let mut cant:i32=0;
    for i in arreglo{
        if i>=inferior&&i<=superior{cant+=1}
    }   
    cant
}
#[test]
fn test_cantidad_en_rango(){
    let arreglo=[80,3,88,120];
    let inf=70;
    let sup=100;
    let cant=cantidad_en_rango(arreglo,inf,sup);
    assert_eq!(2,cant);
}