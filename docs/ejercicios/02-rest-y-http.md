# Ejercicios: REST y semántica HTTP

**Estado:** draft

## Ejercicio 1: nombrar la frontera

Un cliente necesita consultar una factura y solicitar su cancelación. Propón
las URIs y métodos para ambas interacciones. Explica por qué una ruta como
`POST /cancelarFactura` pierde información del contrato.

**Criterio de salida:** la URI nombra un recurso y el método expresa la
intención protocolaria sin depender de un verbo interno.

## Ejercicio 2: decidir un reintento

Un cliente crea un pago y pierde la conexión antes de recibir respuesta.
Decide si puede reintentar con `RetryPolicy::Never`, `ByMethod` o
`IdempotencyKey`. Explica qué efecto duplicado se evita y qué información debe
conservar el proveedor.

**Criterio de salida:** el reintento se justifica por el efecto observable,
no por el deseo de que la solicitud "funcione".

## Ejercicio 3: hacer observable trabajo asíncrono

Una exportación puede tardar minutos. Declara una interacción con `POST`,
`202 Accepted`, `RetryPolicy::IdempotencyKey` y una `FollowUpResource` bajo
`/operaciones/`. Compílala con:

```sh
cargo run --example 02-rest-y-http-solucion
```

**Criterio de salida:** el consumidor recibe una URI que puede consultar sin
suponer que la exportación terminó dentro de la primera respuesta.

## Antes de consultar la solución

- ¿el método elegido es seguro o idempotente por definición?
- ¿qué estado permite al consumidor decidir si debe reintentar o esperar?
- ¿qué promesa pública hace sostenible repetir la misma intención?
