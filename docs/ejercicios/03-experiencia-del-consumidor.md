# Ejercicios: errores, validación y paginación

**Estado:** draft

## Ejercicio 1: error que permite corregir

Una API rechaza el importe de un pago cuando es cero. Declara un código estable,
mensaje y detalle de validación. Explica qué debe evitarse en el detalle para no
exponer información interna.

**Criterio de salida:** una interfaz puede localizar el campo y corregirlo sin
interpretar texto libre ni recibir secretos.

## Ejercicio 2: continuación estable

Una colección de pagos se ordena por `created_at asc, id asc`. Declara una
`Page` de dos elementos con límite dos y cursor opaco. Explica por qué el
consumidor no debe construir el cursor.

**Criterio de salida:** el recorrido tiene orden declarado y continuación que
el consumidor puede reutilizar sin conocer el almacenamiento.

## Ejercicio 3: solución ejecutable

Implementa ambos casos y compila con:

```sh
cargo run --example 03-experiencia-del-consumidor-solucion
```

Antes de consultar la solución, justifica qué acción habilita el código de
error y qué cambio de datos podría romper una paginación sin orden estable.
