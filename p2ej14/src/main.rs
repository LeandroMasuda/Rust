/*14-Definir una función llamada incrementar que recibe como parámetro un número flotante
e incrementa en 1 su valor */


fn incrementar( num:&mut f32){
    *num+=1.0;

}


#[test]
fn test_incrementar(){
    let mut flo=3.0;
    incrementar(&mut flo);
    println!("Resultado {}",flo);
    assert_eq!(flo,4.0);
}