# graficas-Ray-Tracer-Cube

Raytracer en Rust que renderiza un cubo con iluminación **difusa** (sin componente especular) y una **cámara orbital** controlable con el teclado.


## Requisitos

- [Rust y Cargo](https://www.rust-lang.org/tools/install) (edición 2021 o superior)

## Cómo correrlo

```bash
git clone git@github.com:her24770/graficas-Ray-Tracer-Cube.git
cd graficas-Ray-Tracer-Cube
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
src/
├── main.rs          # Loop de render, cast de rayos y shading difuso
├── camera.rs         # Cámara orbital (eye, center, up + basis_change/orbit)
├── cube.rs           # Intersección rayo-cubo (AABB por el método de slabs)
├── ray_intersect.rs  # Trait RayIntersect, Material e Intersect
├── light.rs           # Luz puntual (posición, color, intensidad)
├── color.rs           # Tipo Color con operaciones de suma y escalado
└── framebuffer.rs      # Buffer de píxeles y escritura a la ventana
```

## Modelo de iluminación

Cada material define un color difuso y un albedo. El color final de un píxel se calcula solo con el término **difuso** de Lambert:

```
color = material.diffuse * (ambient + max(dot(normal, luz), 0) * intensidad_luz) * material.albedo
```

No hay componente especular, reflexiones ni sombras: el objetivo de esta etapa es únicamente iluminación difusa sobre una figura geométrica (el cubo).

## Personalización rápida

En `main.rs`:

- **Color del cubo**: `Material::new(Color::new(r, g, b), albedo)`.
- **Fondo**: función `sky_color`, define los colores `top` y `bottom` del degradado.
- **Posición/intensidad de la luz**: `Light::new(posición, color, intensidad)`.
- **Cámara inicial**: `Camera::new(eye, center, up)`.
