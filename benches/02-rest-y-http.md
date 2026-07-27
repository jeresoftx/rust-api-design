# Decisión de benchmark: REST y semántica HTTP

**Estado:** draft

## Pregunta de costo

¿El costo de crear `HttpInteraction` cambia una decisión del consumidor o de
la API que este capítulo busca enseñar?

## Decisión

No se agrega benchmark ejecutable en este capítulo. El modelo solo valida una
combinación pequeña de enums y una URI de seguimiento en memoria. No hay
serialización, handler, caché, red ni carga concurrente cuya medición pueda
orientar una elección de diseño real.

Un benchmark sintético mediría detalles de asignación de una máquina concreta,
pero no demostraría que una API es más segura, más idempotente o más clara.

## Cuándo medir después

La medición será pertinente cuando el curso introduzca una hipótesis concreta:
latencia de serialización, costo de validación bajo lotes, efectividad de una
caché o comportamiento de paginación con datos reproducibles. Cada benchmark
deberá declarar la decisión que puede cambiar con su resultado.
