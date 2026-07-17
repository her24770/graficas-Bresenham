# graficas-Bresenham

Implementación del algoritmo de Bresenham para dibujar líneas sobre un framebuffer, usando solo aritmética entera. Proyecto de la clase de Gráficas por Computadora.

## Estructura

- `src/framebuffer.rs` — buffer de píxeles en memoria (RGB) con `set_pixel`.
- `src/line.rs` — `draw_line(fb, x0, y0, x1, y1)`, algoritmo de Bresenham (variante simétrica) que cubre los 8 octantes.
- `src/bmp.rs` — escritor de archivos BMP (24 bpp) sin dependencias externas.
- `src/main.rs` — arma el framebuffer, dibuja líneas de prueba en todos los octantes y genera `out.bmp`.

## Uso

```bash
cargo run
```

Genera `out.bmp` en la raíz del proyecto.
