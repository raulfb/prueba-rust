
// fn prueba(){
//     println!("Esto es una prueba");
// }

//

// fn pinta_numero(num:i32){
//     for i in 0..num{
//         println!("Numero {}",i+1)
//     }
// }

fn sumar (num:i64,num2:i64)->i64{
    let suma= num+num2;
    //return suma
    suma
    // println!("{}", resultado);

}

fn restar (num:i64,num2:i64)-> i64{
    let resta =num -num2;
    // return resta
    resta
    // println!("{}",resultado)
}

fn multiplicar(num:i64,num2:i64)-> i64{
    let multiplicacion:i64=num*num2;
    //return multiplicacion;
    multiplicacion
    //println!("{}",resultado)
}

fn dividir(num:i64,num2:i64)->i64{
    let division:i64=num/num2;
    // return division
    division
    // println!("{}",resultado)
}

fn modulo(num:i64,num2:i64)->i64{
    let resultado:i64=num%num2;
    //return resultado
    resultado
}
fn main() {
    let numero:i64=5;
    let numero2:i64= 2;

    let resultado_suma=sumar(numero,numero2);
    println!("Resultado de la suma: {} + {} es {}",numero,numero2,resultado_suma);

    let resultado_resta=restar(numero,numero2);
    println!("Resultado de la resta: {} - {} es {}",numero,numero2,resultado_resta);

    let resultado_multiplicacion=multiplicar(numero,numero2);
    println!("Resultado de la multiplicación: {} * {} es {}",numero,numero2,resultado_multiplicacion);

    let resultado_division=dividir(numero,numero2);
    println!("Resultado de la división: {} / {} es {}",numero,numero2,resultado_division);


    let resultado_modulo=modulo(numero,numero2);
    println!("Resultado del modulo: {} % {} es {}",numero,numero2,resultado_modulo);

    // restar(numero,numero2);
    // multiplicar(numero, numero2);
    // dividir(numero, numero2);
    // modulo(numero,numero2);

    // let tupla =(100,89,10.5,999,"hola",[122,444,222,111]);
    // let (x,y,z,w,l,a)=tupla;
    // //let primer_valor=tupla.3;
    // let ultimo_valor= tupla.5;
    // println!("{}",ultimo_valor[2]);
    // //println!("Primer valor: {} Segundo valor: {} Tercer valor: {} Cuarto valor: {}",x,y,z,w);
}
