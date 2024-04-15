/*9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la
suma de los valores del arreglo. */

fn main(){
    let arreglo=[3,2,6,7,9];
    let mut total:u8=0;

    /*for i in arreglo.iter(){
        total+=i;
    }*/
    //SIN estructura
    total=arreglo[0]+arreglo[1]+arreglo[2]+arreglo[3]+arreglo[4];
    println!("La suma del arreglo es {}",total);
}