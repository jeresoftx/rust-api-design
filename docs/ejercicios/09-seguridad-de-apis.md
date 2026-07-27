# Ejercicios: seguridad de APIs

**Estado:** draft

## Ejercicio 1: localizar la frontera

Para una entrada de importe, identifica qué debe validarse antes de ejecutar
la operación, qué detalle no debe regresar al consumidor y qué señal puede
registrarse sin conservar datos sensibles.

## Ejercicio 2: declarar un rechazo seguro

Construye una frontera para `importe` con sensibilidad `Sensitive` y
`SafeMessage`. Explica por qué un detalle interno puede ayudar al atacante aun
cuando el valor de entrada parece válido.

## Ejercicio 3: solución ejecutable

Compila la frontera segura con:

```sh
cargo run --example 09-seguridad-de-apis-solucion
```
