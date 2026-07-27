# Candidata de publicación

**Curso:** API Design en Rust  
**Estado:** `draft`  
**Alcance:** contenido preparado para revisión humana, no publicación

## Evidencia técnica requerida

La candidata debe aprobar estas verificaciones desde la raíz del repositorio:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --all-targets
mdbook build
```

## Decisión pendiente

Una persona debe revisar el contenido, ejercicios, soluciones y navegación antes
de decidir cualquier avance editorial. Esta ficha no cambia el estado del curso
a `reviewed` ni a `published`; la automatización no puede hacerlo.

## Trazabilidad

- RFC-0001 §2: calidad sobre velocidad.
- RFC-0001 §14: estructura de los capítulos.
- RFC-0001 §20: la IA acelera y el criterio humano decide.
