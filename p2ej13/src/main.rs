/*13-Definir una función llamada ordenar_nombres que recibe un arreglo de String y los
ordena en orden alfabético. */


fn ordenar_nombres(stri:&mut[String;3]){
    stri.sort();
}


#[test]

fn test_ordenar_nombre(){
    let mut arreglo=["adbc".to_string(),"zcde".to_string(),"bdaj".to_string()];
    ordenar_nombres(&mut arreglo);
    assert_eq!(arreglo,["adbc","bdaj","zcde"]);
}