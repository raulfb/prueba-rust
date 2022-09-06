
// fn prueba(){
//     println!("Esto es una prueba");
// }
fn pinta_numero(num:i32){
    for i in 0..num{
        println!("Numero {}",i+1)
    }
}

fn main() {
    // println!("Hello, world!");
    //prueba();
    let numero=4;
    pinta_numero(numero);
    println!("El numero es: {}",numero)

}
