# Ejercicios: operación de APIs

**Estado:** draft

## Ejercicio 1: separar frescura y seguridad

Clasifica la política de un catálogo público, un saldo y una actualización de
dirección. Para cada capacidad, decide si puede usar cache compartido, qué
frescura tolera y si una falla transitoria autoriza un reintento automático.

## Ejercicio 2: declarar una actualización honesta

Diseña una política para actualizar una dirección: usa `NoStore`, un límite de
solicitudes recuperable y `NonIdempotent` con `Never`. Explica qué debería ver
el consumidor cuando vence el timeout y por qué no puede asumir que la
actualización falló por completo.

## Ejercicio 3: solución ejecutable

Construye la política de actualización y compila con:

```sh
cargo run --example 07-operacion-de-apis-solucion
```

Antes de consultar la solución, responde qué garantía adicional permitiría
convertir la actualización en una operación segura de reintentar.
