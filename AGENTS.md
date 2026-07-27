# AGENTS.md

Este repositorio es parte de Jeresoft Academy y se rige por RFC-0001.

## Objetivo

Crear el mejor recurso educativo posible sobre diseño de APIs en Rust.

## Antes de escribir código

Siempre, en este orden (RFC-0001 §2 y §13):

1. Explicar el concepto.
2. Explicar el problema.
3. Comparar alternativas.
4. Justificar la implementación.

## Código y documentación

- Rust idiomático, `rustfmt` y Clippy limpios.
- Sin `unsafe` ni dependencias externas no triviales sin justificación escrita.
- Cada capítulo sigue RFC-0001 §14 y §16.
- Todo material visible usa español es-MX.
- Ejemplos, pruebas y benchmarks se agregan cuando aporten evidencia; si un
  benchmark no aplica, se declara por qué.

## GitHub y revisión

- Cada issue accionable tiene assignee, milestone y labels coherentes.
- Cada PR resuelve un issue, tiene un commit principal y usa `Closes #N`.
- No marcar contenido como `reviewed` o `published` sin revisión humana.
- La revisión diferida sigue RFC-0001 §20.
