/*6-Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna
un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
arreglo. */


fn longitud_de_cadenas(a:[&str;4])->[usize;4]{
    let mut cant:usize;
    let mut a1:[usize;4]=[0;4];
    let mut con=0;
    for i in a{
        cant=i.len();
        a1[con]=cant;
        con+=1;
    }
    a1
    
}

#[test]
fn test_longitud_de_cadenas(){
    let a=["as","53f","ds","f32"];
    let a1=longitud_de_cadenas(a);
    assert_eq!(a1,[2,3,2,3]);
}