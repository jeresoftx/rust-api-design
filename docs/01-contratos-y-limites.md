# Contratos y límites de una API

**Estado:** draft

## Introducción

Una API existe en la frontera entre un sistema y sus consumidores. No es solo
una ruta HTTP, una operación GraphQL o un método generado por gRPC: es una
promesa que otra persona, servicio o interfaz usará para tomar decisiones.

Esa promesa necesita ser precisa sin convertir los detalles internos del
sistema en deuda pública. Este capítulo fija el vocabulario y los límites que
usará el resto del curso antes de hablar de estilos, herramientas o código.

## Concepto

Un contrato de API es el acuerdo observable entre quien ofrece una capacidad y
quien la consume. Describe qué se puede pedir, qué respuesta se recibe, qué
errores son significativos y qué reglas permanecen estables durante la vida
útil de esa integración.

Un contrato útil no enumera cómo está construido el proveedor. Enumera lo que
el consumidor puede observar y en qué condiciones puede confiar. Por ejemplo,
una API de pedidos puede prometer que un pedido confirmado incluye un
identificador estable y un estado válido. No necesita prometer la tabla, cola,
servicio o estructura de Rust que el proveedor usa para conseguirlo.

Por eso un contrato tiene cuatro partes inseparables:

- **capacidad:** la intención que el consumidor puede solicitar;
- **forma:** datos, nombres y estructuras que cruzan la frontera;
- **semántica:** significado de una respuesta, un error o una transición;
- **evolución:** cambios que el consumidor puede absorber sin romperse.

## Problema

Sin un contrato explícito, los consumidores terminan deduciendo reglas desde
ejemplos accidentales. Un campo que "siempre venía", un orden que "parecía
estable" o un error que "nunca ocurría" se convierten en dependencias reales
sin que nadie las haya aceptado como públicas.

El daño aparece cuando el proveedor cambia internamente: una optimización,
una migración o una nueva regla de negocio rompe una interfaz que jamás fue
nombrada. El cambio puede ser correcto dentro del servicio y, al mismo tiempo,
ser una regresión para sus consumidores.

El problema no se resuelve publicando cada detalle. Una API que expone
persistencia, modelos internos o decisiones transitorias queda rígida. El
objetivo es seleccionar una frontera pequeña, comprensible y estable: lo
suficiente para integrar con confianza y no más de lo que el proveedor puede
sostener.

## Alternativas

La primera alternativa es diseñar desde la implementación. Parte de las
estructuras existentes y las serializa casi directamente. Es rápida al inicio,
pero hace pública la forma actual del sistema y convierte cada refactor en un
riesgo de compatibilidad.

La segunda es diseñar solo desde la conveniencia del primer consumidor. Puede
producir una integración agradable para un caso concreto, pero suele mezclar
necesidades de pantalla, permisos temporales y supuestos que otros
consumidores no comparten.

La tercera es declarar contratos abstractos sin ejemplos ni reglas de error.
Evita algunos detalles internos, pero deja ambigüedad: dos equipos pueden leer
la misma frase y construir integraciones incompatibles.

Este curso adopta una cuarta alternativa: diseñar la capacidad desde el
consumidor, describir sus resultados observables y poner límites explícitos a
lo que no se promete. La implementación se elegirá después y podrá cambiar
sin reescribir el acuerdo público.

## Qué pertenece al contrato

Un contrato debe explicar aquello que un consumidor necesita para integrar y
operar de forma correcta:

- qué capacidad representa cada operación;
- qué datos acepta y cuáles valida;
- qué datos devuelve y qué significan;
- qué errores puede distinguir y qué acción razonable permiten tomar;
- qué orden, identidad, paginación, concurrencia o permisos son observables;
- qué cambios preservan compatibilidad y cuáles requieren una transición.

La frase guía es sencilla: si un consumidor razonable debe depender de una
regla para usar la API correctamente, esa regla debe aparecer en el contrato.

## Qué debe permanecer interno

No todo comportamiento del proveedor merece convertirse en una promesa. Deben
permanecer internos, salvo que se declaren de forma deliberada, los nombres de
tablas, la topología de servicios, los algoritmos de selección, el orden de
ejecución de procesos auxiliares y la representación exacta de modelos de
dominio.

También debe tratarse con cuidado cualquier dato que sea cómodo para un
consumidor actual pero difícil de sostener a futuro. Publicar una conveniencia
sin nombrar su semántica suele crear una dependencia silenciosa.

## Invariantes

- Cada operación expresa una capacidad, no un detalle de almacenamiento.
- Cada campo público tiene significado, reglas de presencia y límites claros.
- Un consumidor no necesita conocer la implementación para usar el contrato.
- Un error público permite distinguir una situación accionable, no solo que
  "algo salió mal".
- Las reglas que afectan compatibilidad se declaran antes de cambiar la API.
- Lo que no se promete explícitamente no debe asumirse estable.
- La claridad del contrato tiene prioridad sobre la cantidad de datos expuestos.

## Preguntas de diseño

Antes de implementar una API, conviene poder responder estas preguntas:

1. ¿Qué tarea real resuelve el consumidor al invocar esta operación?
2. ¿Qué necesita saber para distinguir éxito, error recuperable y error final?
3. ¿Qué parte de la respuesta es una promesa y qué parte es incidental?
4. ¿Qué cambio interno debería ser posible sin obligar a modificar al
   consumidor?
5. ¿Qué decisión quedaría ambigua si solo existiera un ejemplo feliz?

Estas preguntas no sustituyen una especificación. Evitan que la especificación
empiece demasiado tarde, cuando los consumidores ya convirtieron supuestos en
dependencias.

## Siguiente paso

El modelo Rust del capítulo representará una declaración mínima de capacidad,
su entrada, resultados y límites. No intentará construir un servidor; hará
visible la diferencia entre una promesa pública y un detalle privado. Después,
el capítulo añadirá ejemplos, pruebas y ejercicios para convertir estas
invariantes en evidencia ejecutable.

## Decisiones registradas

- El curso trata el contrato como una promesa observable, no como la
  serialización de un modelo interno.
- Cada capítulo parte de capacidad, semántica y límites antes de elegir una
  tecnología de API.
- El contenido permanece en `draft`; no está revisado ni publicado.
