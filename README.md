# graficas-Bresenham

Proyectos de la clase de Gráficas por Computadora: dibujo de líneas con Bresenham y relleno de polígonos con Scanline Fill, sobre un framebuffer propio.

## Estructura

- `src/framebuffer.rs` — buffer de píxeles en memoria (RGB) con `set_pixel`.
- `src/line.rs` — `draw_line(fb, x0, y0, x1, y1)`, algoritmo de Bresenham (variante simétrica, aritmética entera) que cubre los 8 octantes.
- `src/polygon.rs` — `fill_polygon(fb, contornos, color)`, relleno con Scanline Fill y regla par-impar (soporta varios contornos a la vez, lo que permite resolver agujeros); `draw_polygon_outline(fb, vertices, color)` dibuja el borde reutilizando `draw_line`.
- `src/bmp.rs` — escritor de archivos BMP (24 bpp) sin dependencias externas.
- `src/main.rs` — arma el framebuffer, dibuja los 5 polígonos del laboratorio (el Polígono 5 es un agujero dentro del Polígono 4) y genera `out.bmp` y `out.png`.

## Requisitos

- Tener Rust y Cargo instalados.
- Conexión a internet la primera vez que se compile, para descargar la dependencia `image` (usada solo para exportar el PNG). Las versiones exactas quedan fijadas en `Cargo.lock`, incluido en el repositorio.

## Uso

```bash
cargo run
```

Genera `out.bmp` y `out.png` en la raíz del proyecto. La codificación de PNG usa la crate `image` solo para empaquetar el archivo final; el framebuffer, las líneas y el relleno de polígonos son implementación propia.
