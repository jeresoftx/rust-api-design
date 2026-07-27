# Ejercicios: estilos alternativos

**Estado:** draft

## Ejercicio 1: identificar la interacción

Clasifica cada necesidad según la interacción dominante: una aplicación móvil
que compone perfil, pedidos y recomendaciones; un servicio de inventario que
notifica cambios continuos a otro servicio; y un portal externo que consulta
un recurso de pago. Para cada caso, explica por qué GraphQL, gRPC o REST puede
ser una primera hipótesis y qué límite operativo exige.

## Ejercicio 2: declarar una llamada tipada

Diseña una llamada interna para reservar inventario usando `Grpc`,
`TypedServiceCall` y `RequestBoundary`. Antes de programar, explica qué error,
reintento o regla de compatibilidad tendría que documentarse en un contrato
gRPC real aunque el modelo no los represente todavía.

## Ejercicio 3: solución ejecutable

Construye una `StyleSelection` para la reserva de inventario y compila con:

```sh
cargo run --example 06-estilos-alternativos-solucion
```

Antes de consultar la solución, responde qué cambiaría si la reserva tuviera
que emitir actualizaciones continuas en lugar de una respuesta única.
