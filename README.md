# Proyecto para probar rust

## Instalar rust en Linux
1) Instalar las dependencias necesarias:

```
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

2) Instalar Rust

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Comandos rust

### Ver versión rust

```
rustc --version
```

### Actualizar Rust

```
rustup update
```

### Ver versión cargo

```
cargo --version
```

### Compilar archivo

```
rustc nombreArchivo.rs
```

### Crear proyecto con cargo

```
cargo new prueba-cargo
```

### Construir y correr programa con cargo

```
cargo run
```

### nightly version.
```
rustup default nightly
```

### Tipos de datos

#### Tipos enteros

Para numeros enteros positivos y negativos:
    i8
    i16
    i32
    i64
    isize

Para solamente numeros enteros positivos:
    u8
    u16
    u32
    u64
    u128
    usize

#### Tipos flotantes
    f32
    f64

#### Tipo booleano
    bool

#### Tipo Caracter
    char
    Ejemplo: let caracter:char ='R';


### Librerias interesantes

Rocket: para crear un servidor web
Serde: para serializar y deserializar estructureas de datos de Rust
Diesel: ORM


### Administración memoria en rust

#### Stack(Pila)

Almacena los valores en el orden en el que los obtiene. Elimina los valores en el orden opuesto(elimina primero el ultimo). Todos los datos en el Stack deben de tener un tamaño conocido.
Los enteros se almacenan en el Stack.

#### Heap(Montón)

Almacena los datos que tengan un tamaño desconocido en el momento de la compilacion o con un tamaño que pueda cambiar.
El montón está menos organizado: cuando pones datos en el monton, solicitas una cierta cantidad de espacio. El asignador de memoria
encuentra un lugar vació en el montón que sea lo suficientemente grande, lo marca como en uso y devuelve un puntero, que es la
dirección de esa ubicación. Pj: los strings se almacenan en el heap.

Reglas de propiedad

1) Cada valor en Rust tiene una variable que se llama propietario.
2) Solo puede haber un propietario a la vez.
3) Cuando el propietario se sale del ámbito, el valor se eliminará.

