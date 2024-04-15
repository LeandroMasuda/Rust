/*10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego
cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos
originales. */

fn main(){
    let arreglo1=[2,4,6,8,10];
    let arreglo2=[1,3,5,7,9];
    let mut arreglo3 :[u8 ;5]=[0;5];
    /*
    let mut i=0;
    let mut suma=0;
    while i<5{
        suma=arreglo1[i]+arreglo2[i];
        arreglo3[i]=suma;
        suma=0;
        i+=1;
    }
    */
    //SIN estructura de control
    arreglo3[0]=arreglo1[0]+arreglo2[0];
    arreglo3[1]=arreglo1[1]+arreglo2[1];
    arreglo3[2]=arreglo1[2]+arreglo2[2];
    arreglo3[3]=arreglo1[3]+arreglo2[3];
    arreglo3[4]=arreglo1[4]+arreglo2[4];
    println !("Arreglo 1 {:?}",arreglo1);
    println !("Arreglo 2 {:?}",arreglo2);
    println !("Arreglo sumado {:?}",arreglo3);
}