
// fn prueba(){
//     println!("Esto es una prueba");
// }

//

// fn pinta_numero(num:i32){
//     for i in 0..num{
//         println!("Numero {}",i+1)
//     }
// }


// fn sumar (num:i64,num2:i64)->i64{
//     let suma= num+num2;
//     //return suma
//     suma
//     // println!("{}", resultado);

// }

// fn restar (num:i64,num2:i64)-> i64{
//     let resta =num -num2;
//     // return resta
//     resta
//     // println!("{}",resultado)
// }

// fn multiplicar(num:i64,num2:i64)-> i64{
//     let multiplicacion:i64=num*num2;
//     //return multiplicacion;
//     multiplicacion
//     //println!("{}",resultado)
// }

// fn dividir(num:i64,num2:i64)->i64{
//     let division:i64=num/num2;
//     // return division
//     division
//     // println!("{}",resultado)
// }

// fn modulo(num:i64,num2:i64)->i64{
//     let resultado:i64=num%num2;
//     //return resultado
//     resultado
// }

// fn palabra()-> String {
//     let hello = String::from("Hello, world!");
//     return hello;

// }

// fn modifica(cadena:&mut String){
//     cadena.push_str(" amigos !")
//     // println!("{}",cadena);
//     // cadena
// }

// struct Usuario{
//     nombre:String,
//     email:String,
//     edad:u32,
//     activo:bool
// }

// fn crear_usuario(nombre:String,email:String,edad:u32) -> Usuario{
//     Usuario{
//         nombre,
//         email,
//         edad,
//         activo:true,
//     }

// }

// struct Rectangulo{
//     ancho:u32,
//     alto:u32
// }

// impl Rectangulo{
//     fn area(&self) ->u32{
//         self.ancho * self.alto
//     }

//     fn puede_contener(&self,otro:&Rectangulo)->bool{
//         self.ancho>otro.ancho && self.alto >otro.alto
//     }

// }

// use core::panicking::panic;
use std::fs::File;


fn main() {
    let f = File::open("hola.txt");

    let f = match f {
        Ok(fichero)=>fichero,
        Err(error)=> panic!("Error abriendo el fichero: {:?}",error),
    };
    println!("{:?}",f);

    // let vector = vec![100,200,300,400,500];
    // let aux = vector[9];

    // use std::collections::HashMap;

    // let mut puntuaciones = HashMap::new();
    // // puntuaciones.insert(String::from("Warriors"), 121);
    // // puntuaciones.insert(String::from("Warriors"), 244);
    // puntuaciones.insert(String::from("Lakers"), 104);
    // puntuaciones.entry(String::from("Warriors")).or_insert(1000);

    // println!("{:?}",puntuaciones);

    // let nombre_equipo = String::from("Warriors");
    // let puntuacion= puntuaciones.get(&nombre_equipo);

    // println!("{}", puntuacion);
    // let array_prueba=[112,33,44,22,33];
    // println!("{:?}",array_prueba);
    // for (clave,valor) in &puntuaciones {
    //     println!("{}: {}",clave,valor);
    // }

    // let numero=9;
    // let maximo_configurado= Some(numero);
    // if let Some(maximo) = maximo_configurado{
    //     println!("El maximo configurado es: {}", maximo);
    // }

    // let mut v: Vec<i32>= Vec::new();
    // v.push(5);
    // v.push(10);
    // v.push(98);
    // v.pop();
    // println!("La longitud del vector es: {}",v.len())

    // let rectangulo1 = Rectangulo{
    //     ancho:20,
    //     alto:40,
    // };

    // let rectangulo2= Rectangulo{
    //     ancho:30,
    //     alto:50
    // };

    // let rectangulo3=Rectangulo{
    //     ancho:10,
    //     alto:20
    // };

    // println!("El área del rectángulo es: {}",rectangulo1.area());
    // println!("¿El rectangulo 1 puede contener el rectangulo 2? {}", rectangulo1.puede_contener(&rectangulo2));
    // let usuario2 =crear_usuario(String::from("Rauwwwl"), String::from("raul@example.com"), 26);
    // println!("{}",usuario2.nombre)
    // println!("{}",usuario2.nombre);
    // let mut usuario1 = Usuario{
    //     nombre: String::from("Raul"),
    //     email: String::from("raul@example.com"),
    //     edad:26,
    //     activo:true
    // };
    // usuario1.nombre= String::from("pepe");
    // println!("{}",usuario1.nombre);
    //1..=10 del 1 al 10
    // for numero in (1..=10).rev(){
    //     println!("{}",numero);
    // }

    // let j=3;
    // let y=j;

    // println!("{}",y);
    // println!("{}",j)

    // let mut cadena= String::from("Hola");
    // modifica(&mut cadena);
    // // println!("La longitud de la cadena es: {}",y);
    // println!("La cadena es: {}",cadena)
    // let cadena2 = cadena.clone();
   
    // println!("{}",cadena2);
    // // let cadena =String::from("EEEEE11111");
    // // println!("cadena {}",cadena);
    // println!("{}",cadena);

    
    //let cadena = String::from("holaaa");
    //println!("{}",cadena)
    
    // let mut indice =0;

    // while indice < 6{
    //     println!("El valor es: {}",array_numeros[indice]);
    //     indice +=1;    
    // }
    // let resultado= loop {
    //     // println!("Hola");
    //     // println!("{}",contador);
        
    //     contador += 1;
        
    //     if contador >= 10{
    //         break contador;
    //     }
    // };
    // println!("el resultado es: {}",resultado)

    // let numero:i64=5;
    // let numero2:i64= 4;

    // if numero2 == 1 {
    //     println!("Es igual a  uno");
    // }else if numero2 == 2{       
    //     println!("Es igual a dos");
    // }else{
    //     println!("Else");
    // }

    // let numero2:i64= 4;

    // let numero3 = if numero2 == 4 { 22 } else {12};
    
    // println!("El valor es:  {}",numero3);



    // let resultado_suma=sumar(numero,numero2);
    // println!("Resultado de la suma: {} + {} es {}",numero,numero2,resultado_suma);

    // let resultado_resta=restar(numero,numero2);
    // println!("Resultado de la resta: {} - {} es {}",numero,numero2,resultado_resta);

    // let resultado_multiplicacion=multiplicar(numero,numero2);
    // println!("Resultado de la multiplicación: {} * {} es {}",numero,numero2,resultado_multiplicacion);

    // let resultado_division=dividir(numero,numero2);
    // println!("Resultado de la división: {} / {} es {}",numero,numero2,resultado_division);


    // let resultado_modulo=modulo(numero,numero2);
    // println!("Resultado del modulo: {} % {} es {}",numero,numero2,resultado_modulo);

    // let resultado_palabra=palabra();
    // println!("{}",resultado_palabra);

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
