# Ejercicios: identidad y acceso

**Estado:** draft

## Ejercicio 1: separar preguntas

Para una credencial válida de Ana, distingue qué verifica la audiencia y qué
decide la autorización al leer o actualizar un pago. Explica qué detalle no
debe revelarse cuando Ana intenta modificar un recurso ajeno.

## Ejercicio 2: declarar una lectura permitida

Construye una solicitud para que Ana lea `payment:123` en `payments-api`.
Antes de programar, identifica sujeto, acción, recurso y la capacidad que se
concede sin convertir la sesión en permiso general.

## Ejercicio 3: solución ejecutable

Compila la lectura autorizada con:

```sh
cargo run --example 08-identidad-y-acceso-solucion
```
