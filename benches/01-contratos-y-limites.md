# Decisión de benchmark: contratos y límites de una API

**Estado:** draft

## Pregunta de costo

¿Construir y validar un `ApiContract` es una ruta cuyo tiempo de ejecución
deba guiar una decisión de diseño del capítulo?

## Decisión

No se agrega benchmark ejecutable en este capítulo.

`ApiContract` es un modelo educativo creado en memoria. Su validación recorre
las entradas y salidas para detectar nombres duplicados mediante un
`BTreeSet`; para `n` campos, esa parte tiene costo `O(n log n)`. El tamaño de
los contratos de los ejercicios es pequeño y el resultado no representa una
carga de producción, una serialización ni una llamada de red.

Medir este constructor sin una hipótesis de producto solo produciría números
de una máquina y una asignación de memoria concreta. No permitiría decidir
entre diseños de API ni enseñaría una compensación relevante al consumidor.

## Señal que justificaría medir después

Un benchmark será apropiado cuando un capítulo introduzca una ruta real con
una pregunta comparativa, por ejemplo:

- serializar respuestas grandes bajo un formato específico;
- validar lotes de solicitudes con límites de tamaño definidos;
- comparar estrategias de paginación o caching con una carga reproducible.

En ese momento la medición deberá declarar datos de entrada, hipótesis,
métrica, entorno y decisión que puede cambiar. Hasta entonces, esta nota evita
confundir la ejecución de un benchmark con evidencia útil.
