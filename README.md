# graficas-Ray-Tracer-Cube

Raytracer en Rust que renderiza un cubo **texturizado** con iluminación difusa y una **cámara orbital** controlable con el teclado.

## Requisitos

- [Rust y Cargo](https://www.rust-lang.org/tools/install) (edición 2021 o superior)

## Cómo correrlo

```bash
git clone git@github.com:her24770/graficas-Ray-Tracer-Cube.git
cd graficas-Ray-Tracer-Cube
git checkout Ray-Tracer-Cube-Texturizado
cargo run
```

Esto compila el proyecto y abre una ventana de 800x600 con el cubo renderizado.

Para mejor rendimiento (recomendado):

```bash
cargo run --release
```

## Controles

| Tecla | Acción |
|-------|--------|
| ← / → | Orbitar la cámara horizontalmente (yaw) |
| ↑ / ↓ | Orbitar la cámara verticalmente (pitch) |
| Esc   | Cerrar la ventana |

La cámara orbita siempre alrededor del centro del cubo, manteniendo la distancia (radio) fija.

## Estructura del proyecto

```
assets/
└── stone.png          # Textura del cubo (piedra)

src/
├── main.rs             # Loop de render, cast de rayos y shading difuso
├── camera.rs           # Cámara orbital (eye, center, up + basis_change/orbit)
├── cube.rs             # Intersección rayo-cubo (AABB) + cálculo de UV por cara
├── texture.rs          # Carga de imágenes (PNG) y muestreo por coordenadas UV
├── ray_intersect.rs    # Trait RayIntersect, Material e Intersect
├── light.rs            # Luz puntual (posición, color, intensidad)
├── color.rs            # Tipo Color con operaciones de suma y escalado
└── framebuffer.rs      # Buffer de píxeles y escritura a la ventana
```

## Texturizado

Cada cara del cubo calcula sus propias coordenadas UV a partir del punto de impacto y la normal (mapeo tipo *box mapping*): se toman los dos ejes locales distintos al de la normal, normalizados de `[-size/2, size/2]` a `[0, 1]`, y se usan para muestrear la textura en `Texture::sample(u, v)`.

La imagen resultante reemplaza el color difuso plano del material; la iluminación se sigue aplicando igual que antes sobre ese color muestreado.

## Modelo de iluminación

Cada material tiene un color difuso (ahora proviene de la textura) y un albedo. El color final de un píxel se calcula solo con el término **difuso** de Lambert:

```
color = material.diffuse * (ambient + max(dot(normal, luz), 0) * intensidad_luz) * material.albedo
```

No hay componente especular, reflexiones ni sombras.

## Personalización rápida

En `main.rs`:

- **Textura del cubo**: `Texture::load("assets/otra_textura.png")`.
- **Fondo**: constante `BACKGROUND_COLOR`.
- **Posición/intensidad de la luz**: `Light::new(posición, color, intensidad)`.
- **Cámara inicial**: `Camera::new(eye, center, up)`.

## Créditos

La textura `assets/stone.png` viene del repositorio del curso [cc2018-2026-02-10](https://github.com/menene/cc2018-2026-02-10) (rama `11-RC-05-MAZE-TEXTURES`, archivo `wall1.png`).
