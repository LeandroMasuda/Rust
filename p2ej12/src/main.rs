/*12-Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
reemplaza todos los números pares por -1. */

fn reemplazar_pares( arreglo:&mut[i32;4]){
    let mut cont=0;
    for i in *arreglo{
        if i %2==0 {
            arreglo[cont]=-1;
            
        }
        cont +=1;
    }
}

#[test]

fn test_reemplazar_pares(){
    let mut a=[2,3,5,6];
    reemplazar_pares(&mut a);
    assert_eq!(a,[-1,3,5,-1]);
}