/*7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números
enteros, y luego multiplique cada valor del arreglo por un valor constante definido,
modificando el contenido del arreglo. */

fn main(){
    let mut arreglo=[1,2,3,4,5,6,];
    let constante=2;
    println!("Arreglo original{:?}",arreglo);
    arreglo[0]=arreglo[0]*constante;
    arreglo[1]=arreglo[1]*constante;
    arreglo[2]=arreglo[2]*constante;
    arreglo[3]=arreglo[3]*constante;
    arreglo[4]=arreglo[4]*constante;
    arreglo[5]=arreglo[5]*constante;
    println!("Arreglo multiplicado{:?}",arreglo);
}
