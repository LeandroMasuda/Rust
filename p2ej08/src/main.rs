/*8- Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
arreglos pasados por parámetro, correspondiéndose el resultado con cada posición de los
arreglos pasados por parámetro. */


fn sumar_arreglos(a1:[f32;4],a2:[f32;4])->[f32;4]{
    let mut a3:[f32;4]=[0.0;4];
    let mut flo:f32;
    for i in 0..a1.len(){
        flo=a1[i]+a2[i];
        a3[i]=flo;
    }
    a3
}

#[test]

fn test_sumar_arreglos(){
    let a1=[1.0,2.0,3.0,4.0];
    let a2=[1.0,2.0,3.0,4.0];
    let a3:[f32;4]=sumar_arreglos(a1,a2);
    assert_eq!(a3,[2.0,4.0,6.0,8.0]);
}