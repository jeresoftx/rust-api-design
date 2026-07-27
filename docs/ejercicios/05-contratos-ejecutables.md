# Ejercicios: contratos ejecutables

**Estado:** draft

## Ejercicio 1: leer la promesa pública

Para `GET /payments/{payment_id}`, explica qué decisión permite tomar una
respuesta `200` y qué decisión distinta permite tomar una respuesta `404`.
Después identifica qué dato faltaría si el consumidor necesitara distinguir un
pago inexistente de un pago al que no tiene acceso.

## Ejercicio 2: declarar una creación

Diseña `POST /payments` con el identificador de operación `createPayment`.
Declara una respuesta `201` que indique que el pago fue creado y una respuesta
`422` que indique que la entrada es inválida. Antes de programar, justifica por
qué esas respuestas son parte del contrato y no detalles del handler.

## Ejercicio 3: solución ejecutable

Construye una `OperationSpec` para `createPayment` y compila con:

```sh
cargo run --example 05-contratos-ejecutables-solucion
```

Antes de consultar la solución, responde qué prueba de contrato detectaría que
el servidor devolviera `200` en lugar de `201` después de crear un pago.
