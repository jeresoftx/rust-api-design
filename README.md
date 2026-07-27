# API Design en Rust

Repositorio complementario de Jeresoft Academy para diseñar APIs que puedan
evolucionar sin romper a sus consumidores.

El curso estudia REST, GraphQL, gRPC, versionado, paginación, caching, rate
limiting, autenticación, seguridad de aplicaciones y OpenAPI. Rust es el
vehículo para modelar contratos explícitos; el criterio de interfaz es el
objetivo.

## Lugar en el camino

Es un curso complementario técnico de RFC-0001 §10. Parte de diseño modular,
testing y HTTP; alimenta arquitectura, cloud, DevOps y dominios aplicados.

## Estado editorial

El curso está en `draft`. Los capítulos, ejemplos y prácticas están preparados
para revisión humana; el contenido no está publicado. La revisión humana es
obligatoria antes de usar `reviewed` o `published`.

## Cómo verificarlo

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --all-targets
```

## Referencias

- RFC-0001 §10: currículum.
- RFC-0001 §13: estándares de Rust.
- RFC-0001 §14: anatomía de un capítulo.
- RFC-0001 §15: plantilla de repositorio.
