# Ejercicios: contratos y límites de una API

**Estado:** draft

Estos ejercicios continúan el capítulo 1. Antes de escribir Rust, nombra la
capacidad, el consumidor y la promesa observable. No uses una tabla, un ORM o
un framework como punto de partida.

## Ejercicio 1: separar promesa de implementación

Un servicio permite que una tienda consulte el saldo disponible de una tarjeta
de regalo. El equipo actual almacena movimientos en dos tablas y consulta una
caché antes de ir a la base de datos.

Escribe dos listas:

1. tres datos o reglas que el consumidor sí necesita conocer;
2. tres decisiones que deben permanecer privadas.

Después, explica qué regresión produciría exponer el nombre de una tabla como
parte de la respuesta de la API.

**Criterio de salida:** cada elemento público permite al consumidor integrar o
tomar una decisión. Cada elemento privado puede cambiar sin obligar a cambiar
la integración.

## Ejercicio 2: convertir un error en una acción

Una operación de registro de pago puede rechazar el intento porque la factura
no existe, ya está pagada o el importe es inválido. Para cada caso, propone un
código público y la acción que el consumidor puede tomar.

Evita respuestas como "mostrar error". La acción debe distinguir si conviene
corregir una entrada, detener el flujo o consultar el estado actual.

**Criterio de salida:** un consumidor puede programar una decisión diferente
para cada error sin interpretar texto libre.

## Ejercicio 3: declarar un contrato de pago

Modela la capacidad "registrar un pago" con `ApiContract`:

- recibe `factura_id` e `importe` como entradas requeridas;
- devuelve `pago_id` y `estado` como salidas requeridas;
- declara al menos los errores `factura_no_encontrada` y `factura_ya_pagada`;
- deja privado cómo se persiste o concilia el pago.

Compila tu solución con:

```sh
cargo run --example 01-contratos-y-limites
```

**Criterio de salida:** el contrato compila, cada campo tiene significado y
ninguna decisión de almacenamiento aparece como promesa pública.

## Antes de consultar la solución

Comprueba que puedes responder lo siguiente sobre tu diseño:

- ¿qué tarea resuelve el consumidor?
- ¿qué campo o error sería peligroso renombrar?
- ¿qué cambio interno debería ser posible sin modificar el contrato?
- ¿qué regla no está escrita y podría convertirse en una dependencia
  accidental?

La solución no es la única forma válida de representar el dominio. Es una base
para comparar decisiones y discutir sus costos.
