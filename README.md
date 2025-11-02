
# Software Renderer - Laboratorio de Gráficas por Computadora

Un renderizador 3D por software implementado desde cero en Rust, con sistema completo de shaders, cámara orbital, z-buffer y múltiples efectos de iluminación en tiempo real.

## 📋 Descripción

Este proyecto implementa un **motor de renderizado 3D completamente por software** (sin usar GPU), que carga modelos 3D en formato OBJ y los renderiza usando un pipeline gráfico completo. Incluye sistema de vertex shaders, fragment shaders, z-buffer para depth testing, cámara orbital interactiva y múltiples efectos visuales.

## ✨ Características

- **Pipeline gráfico completo**: Vertex Shader → Rasterización → Fragment Shader
- **Carga de modelos OBJ**: Parser personalizado con soporte para vértices, normales y coordenadas de textura
- **Sistema de shaders intercambiables**: 5 shaders diferentes en tiempo real
- **Z-Buffer**: Depth testing correcto para renderizado de superficies ocluidas
- **Cámara orbital**: Sistema de cámara 3D con órbita, zoom y matrices view/projection
- **Rasterización de triángulos**: Coordenadas baricéntricas con interpolación de normales
- **Iluminación avanzada**: Difusa, cel shading, procedural y visualización de normales
- **Framebuffer personalizado**: Sistema de dibujado pixel por pixel con depth buffer

## 🖼️ Modelo Renderizado

![Modelo Renderizado](screenshot.png)
*Nave espacial con diferentes shaders: difuso, cel shading, procedural y normal map*

## 🎮 Controles

### Cámara
| Tecla | Acción |
|-------|--------|
| `W` / `↑` | Orbitar hacia arriba |
| `S` / `↓` | Orbitar hacia abajo |
| `A` / `←` | Orbitar a la izquierda |
| `D` / `→` | Orbitar a la derecha |
| `Q` | Zoom out (alejar) |
| `E` | Zoom in (acercar) |
| `R` | Resetear cámara a posición inicial |

### Shaders
| Tecla | Shader |
|-------|--------|
| `1` | Color estático (amarillo) |
| `2` | Iluminación difusa |
| `3` | Cel Shading (toon shading) |
| `4` | Shader procedural animado |
| `5` | Normal Map (visualización de normales) |

### General
| Tecla | Acción |
|-------|--------|
| `ESC` | Salir de la aplicación |

## 🛠️ Requisitos

- **Rust**: versión 1.70.0 o superior
- **Cargo**: gestor de paquetes de Rust

### Dependencias

```toml
[dependencies]
minifb = "0.27"
nalgebra-glm = "0.18"
tobj = "4.0.2"
```

## 📦 Instalación

1. **Clonar el repositorio**:
```bash
git clone <url-del-repositorio>
cd proyecto2
```

2. **Colocar el modelo OBJ**:
   - Coloca tu archivo `spaceship.obj` en la raíz del proyecto
   - El modelo debe estar en formato OBJ estándar con normales

3. **Compilar el proyecto**:
```bash
cargo build --release
```

4. **Ejecutar**:
```bash
cargo run --release
```

## 📁 Estructura del Proyecto

```
proyecto2/
├── Cargo.toml              # Configuración y dependencias
├── spaceship.obj           # Modelo 3D a renderizar
├── screenshot.png          # Captura del modelo renderizado
└── src/
    ├── main.rs             # Punto de entrada y loop principal de renderizado
    ├── color.rs            # Estructura de color RGB con operaciones
    ├── vertex.rs           # Estructura de vértice con atributos 3D
    ├── fragment.rs         # Estructura de fragmento para rasterización
    ├── framebuffer.rs      # Buffer de píxeles con z-buffer
    ├── triangle.rs         # Rasterización con coordenadas baricéntricas
    ├── shaders.rs          # Sistema de shaders (vertex y fragment)
    ├── camera.rs           # Sistema de cámara orbital 3D
    └── obj_loader.rs       # Parser de archivos OBJ
```

## 🔧 Componentes Técnicos

### 1. **Pipeline Gráfico**

El renderizador implementa un pipeline gráfico completo:

```
Modelo 3D → Vertex Shader → Primitive Assembly → Rasterización → Fragment Shader → Framebuffer
```

#### **Vertex Shader**
- Transforma vértices del espacio objeto al espacio de pantalla
- Aplica matrices: Model → View → Projection → Viewport
- Transforma normales para iluminación
- Realiza perspective division

#### **Rasterización**
- Convierte triángulos en fragmentos
- Usa coordenadas baricéntricas para interpolación
- Calcula profundidad (z) para cada píxel

#### **Fragment Shader**
- Aplica iluminación y efectos por píxel
- 5 shaders diferentes intercambiables
- Usa normales interpoladas para iluminación suave

### 2. **Sistema de Cámara** (`camera.rs`)
- **Cámara orbital**: Orbita alrededor del modelo
- **Matrices de vista**: Look-at matrix para transformación view
- **Proyección perspectiva**: FOV de 45°, near plane 0.1, far plane 1000
- **Controles intuitivos**: WASD para orbitar, QE para zoom

### 3. **Z-Buffer** (`framebuffer.rs`)
- Depth testing para renderizado correcto de superficies
- Resuelve problemas de oclusión
- Buffer de profundidad flotante (f32::INFINITY inicial)

### 4. **Cargador OBJ** (`obj_loader.rs`)
- Parser completo de archivos .obj
- Soporte para vértices (v), normales (vn), coordenadas de textura (vt)
- Manejo de índices v/vt/vn
- Triangulación automática de polígonos
- Normalización y centrado del modelo

### 5. **Sistema de Shaders** (`shaders.rs`)

#### **Shader 1: Color Estático**
```rust
Color base amarillo constante
```

#### **Shader 2: Iluminación Difusa**
```rust
intensidad = 0.3 (ambient) + 0.7 * max(0, dot(normal, luz))
color_final = color_base * intensidad
```

#### **Shader 3: Cel Shading**
```rust
Cuantización de intensidad en 4 niveles:
- > 0.8: intensidad = 1.0 (muy iluminado)
- > 0.5: intensidad = 0.6 (medio iluminado)
- > 0.2: intensidad = 0.4 (sombra suave)
- ≤ 0.2: intensidad = 0.2 (sombra fuerte)
```

#### **Shader 4: Procedural Animado**
```rust
pattern = |sin(x * 0.1 + time) * cos(y * 0.1 + time) * 0.5 + 0.5|
Genera patrones animados basados en posición y tiempo
```

#### **Shader 5: Normal Map**
```rust
Visualiza las normales como colores RGB:
R = (normal.x + 1.0) * 0.5
G = (normal.y + 1.0) * 0.5
B = (normal.z + 1.0) * 0.5
```

## 🎨 Matemáticas y Algoritmos

### Transformaciones 3D

```rust
// Matrices de transformación
Model Matrix:      Escala, rotación y traslación del objeto
View Matrix:       Posición y orientación de la cámara (look-at)
Projection Matrix: Proyección perspectiva (frustum)
Viewport Matrix:   NDC → coordenadas de pantalla
```

### Coordenadas Baricéntricas

```rust
Para un punto P dentro del triángulo ABC:
P = w1*A + w2*B + w3*C
donde w1 + w2 + w3 = 1 y w1, w2, w3 ≥ 0
```

Usado para interpolación de:
- Profundidad (z)
- Normales
- Colores
- Coordenadas de textura

### Iluminación Difusa (Lambertiana)

```rust
I_diffuse = I_light * k_d * max(0, N · L)
donde:
- N = normal de la superficie (normalizada)
- L = dirección hacia la luz (normalizada)
- k_d = coeficiente de reflexión difusa
```

## 🚀 Optimizaciones

- **Compilación optimizada**: `-O3` en modo desarrollo
- **Bounding box**: Solo rasteriza píxeles dentro del área del triángulo
- **Early depth test**: Descarta fragmentos con profundidad mayor
- **Vec capacity**: Pre-aloca memoria para vectores
- **Target FPS**: 60 FPS para rendimiento consistente

## 📊 Especificaciones Técnicas

- **Resolución**: 800 x 600 píxeles
- **Profundidad de color**: 24 bits (8 bits por canal RGB)
- **Z-Buffer**: 32 bits flotante
- **Espacios de coordenadas**: Object → World → View → Clip → NDC → Screen
- **Proyección**: Perspectiva con FOV 45°
- **Culling**: Ninguno (renderiza todas las caras)

## 🐛 Solución de Problemas

### El modelo no se ve
- Verifica que `spaceship.obj` esté en la raíz del proyecto
- Asegúrate de que el archivo OBJ tenga normales (vn)
- Prueba con el shader de normal map (tecla 5) para verificar geometría

### El modelo se ve muy pequeño/grande
- Ajusta el parámetro en `model.normalize_and_center(1.5)`
- Valores sugeridos: 0.5 a 3.0

### Artefactos visuales o z-fighting
- El z-buffer debe estar funcionando correctamente
- Verifica que las normales estén correctamente calculadas

### Rendimiento lento
- Compila en modo release: `cargo run --release`
- Reduce la resolución en `framebuffer.rs`
- Usa modelos con menos polígonos

## 💡 Extensiones Posibles

- [ ] Texturas con UV mapping
- [ ] Múltiples fuentes de luz
- [ ] Specular highlighting (Phong/Blinn-Phong)
- [ ] Shadow mapping
- [ ] Normal mapping real (con texturas)
- [ ] Ambient occlusion
- [ ] Skybox
- [ ] Post-processing effects
- [ ] Carga de múltiples modelos simultáneos

## 👨‍💻 Desarrollo

### Agregar un nuevo shader

1. Agrega la función en `shaders.rs`:
```rust
fn mi_nuevo_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Tu lógica aquí
    Color::new(255, 0, 0)
}
```

2. Agrégalo al match en `fragment_shader`:
```rust
match shader_type {
    // ...
    "mi_shader" => mi_nuevo_shader(fragment, uniforms),
    _ => fragment.color
}
```

3. Agrégalo a los controles en `main.rs`:
```rust
if window.is_key_pressed(Key::Key6, minifb::KeyRepeat::No) {
    current_shader = "mi_shader";
    println!("Shader: Mi Nuevo Shader");
}
```

### Modificar la iluminación

Cambiar dirección de luz en `main.rs`:
```rust
let uniforms = Uniforms {
    // ...
    light_dir: Vec3::new(1.0, -1.0, 0.0), // X, Y, Z
};
```

### Ajustar la cámara inicial

En `main.rs`:
```rust
let mut camera = Camera::new(
    Vec3::new(0.0, 2.0, 8.0),  // Posición (más alto y más lejos)
    Vec3::new(0.0, 0.0, 0.0),  // Target (centro)
    Vec3::new(0.0, 1.0, 0.0),  // Up vector
);
```

## 📚 Recursos de Aprendizaje

- [Learn OpenGL - Coordinate Systems](https://learnopengl.com/Getting-started/Coordinate-Systems)
- [Scratchapixel - Rasterization](https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation)
- [Barycentric Coordinates](https://codeplea.com/triangular-interpolation)
- [The Book of Shaders](https://thebookofshaders.com/)

## 📄 Licencia

Este proyecto fue desarrollado como parte de un laboratorio de Gráficas por Computadora.

## 🙏 Agradecimientos

- **minifb**: Window management y framebuffer
- **nalgebra-glm**: Matemáticas 3D (vectores, matrices)
- **tobj**: Alternativa para carga de OBJ (incluida pero no usada)
- Basado en los principios de rasterización clásica y pipeline gráfico moderno

---

**Fecha de desarrollo**: Noviembre 2025  
**Lenguaje**: Rust 1.70.0+  
**Paradigma**: Software Rendering (CPU-based)  
**Arquitectura**: Pipeline gráfico completo con shaders programables

