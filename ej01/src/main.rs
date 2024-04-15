/*1- Escribir un programa que defina una variable de tipo flotante con algún valor, y luego
permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y
restar su valor. Se deben imprimir los resultados.*/
use std::io::stdin;
fn main(){

    let flo=3.0;
    //loop{
        let mut num =String::new();
        println !("Ingrese un numero para multiplicar");
        stdin().read_line(&mut num).expect("Error al leer numero.");
        let valor:f32=num.trim().parse().expect("Error");
        /*let valor:f32=match num.trim().parse(){
           Ok(num)=>num,
            Err(_)=> continue
        };
        if valor ==0.0{
            break 
        }*/
        let multi= flo*valor;
        println !("Multiplicacion, {}!",multi);
        let divi= flo/valor;
        println !("Division, {}!",divi);
        let suma= flo+valor;
        println !("Suma, {}!",suma);
        let res= flo-valor;
        println !("Resta, {}!",res);
        
    //}   

    

}
