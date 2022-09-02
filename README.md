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


