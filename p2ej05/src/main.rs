/*5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro. */

fn duplicar_valores(arreglo:[f32;4])->[f32;4]{
    let   mut a:[f32;4]=[0.0;4];
    let mut cont=0;
    for i in arreglo{ 
        a[cont]=arreglo[cont]*2.0;
        cont+=1;
    }
    a
}
#[test]
fn test_duplicar_valores(){
    let a:[f32;4]=[1.2,3.4,5.6,6.7];
    let a1:[f32;4]=duplicar_valores(a);
    assert_eq!([2.4,6.8,11.2,13.4],a1);
}