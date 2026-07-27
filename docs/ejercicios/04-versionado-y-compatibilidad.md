# Ejercicios: versionado y compatibilidad

**Estado:** draft

## Ejercicio 1: clasificar el cambio

Clasifica como compatible o incompatible: agregar un campo opcional, eliminar
un campo, cambiar el orden de una página y añadir una operación nueva. Para
cada ruptura, identifica qué decisión previa del consumidor podría cambiar.

## Ejercicio 2: escribir una deprecación

El campo `estado` dejará de representar un pago y será reemplazado por
`payment_status`. Declara el comportamiento deprecado, reemplazo y fecha de
retirada. Explica por qué conservar el nombre y cambiar el significado no es
una evolución compatible.

## Ejercicio 3: solución ejecutable

Construye una `Migration` para `ChangeFieldMeaning` y compila con:

```sh
cargo run --example 04-versionado-y-compatibilidad-solucion
```

Antes de consultar la solución, responde qué evidencia de adopción pedirías
antes de retirar el comportamiento anterior.
