# Errores, validación y paginación

**Estado:** draft

## Introducción

Una API útil no solo responde cuando todo sale bien. También explica qué parte
de una solicitud debe corregirse, qué regla impidió procesarla y cómo recorrer
una colección sin depender de que sus datos permanezcan inmóviles. Esa
experiencia forma parte del contrato, no es un detalle decorativo del cuerpo
JSON.

Este capítulo trata errores, validación y paginación como tres formas de
reducir incertidumbre para el consumidor. Las tres deben ser predecibles,
accionables y estables frente a cambios internos del proveedor.

## Concepto

Un error accionable identifica la categoría de falla, los datos afectados y la
acción razonable que puede tomar el consumidor. No necesita revelar una traza,
una consulta o una excepción interna. Necesita permitir que una interfaz
muestre una corrección, que un cliente automatizado decida reintentar o que un
operador investigue con una referencia segura.

La validación es la aplicación explícita de reglas sobre una solicitud antes de
producir un efecto. Una regla puede referirse a forma, rango, presencia,
permisos o estado de dominio. El consumidor debe distinguir una solicitud que
no se entiende de una solicitud entendida pero inválida según una regla.

La paginación divide una colección potencialmente grande en porciones que un
consumidor puede recorrer. Un cursor no es un desplazamiento interno que se
filtra al exterior: es una continuación opaca con reglas de orden, límite y
consistencia que el contrato debe declarar.

## Problema

Una respuesta como `{"error":"algo salió mal"}` obliga al consumidor a
interpretar texto, adivinar el campo afectado o tratar todos los fallos como
transitorios. Eso crea interfaces frágiles y hace imposible construir una
experiencia clara sin duplicar reglas del servidor.

La validación también se vuelve confusa cuando mezcla errores de sintaxis,
reglas de negocio y fallas internas bajo el mismo estado o formato. El cliente
no sabe si corregir, reintentar o escalar. El proveedor tampoco puede cambiar
el mensaje libre sin romper automatizaciones silenciosas.

En colecciones, usar solo `offset` y `limit` parece simple hasta que llegan
inserciones, eliminaciones o órdenes no deterministas. El consumidor puede ver
duplicados, saltos o resultados que cambian entre páginas. El problema no es
elegir cursor por moda, sino declarar qué recorrido puede confiar el cliente.

## Alternativas

La primera alternativa es devolver mensajes humanos sin código estable. Es
rápida de implementar, pero no sirve para automatizar decisiones ni localizar
una corrección con precisión.

La segunda es usar una lista de errores de validación sin semántica común. Da
más datos, pero cada endpoint termina inventando nombres, formatos y acciones
distintas.

La tercera es paginar con offsets sin declarar orden ni consistencia. Puede
ser adecuada para reportes estáticos pequeños, pero se vuelve ambigua en una
colección que cambia mientras se recorre.

Este curso adopta códigos estables con detalles por campo, reglas de validación
explícitas y continuaciones opacas. No prohíbe `offset`; exige justificarlo
desde la consistencia, tamaño y experiencia que necesita el consumidor.

## Errores como contrato

Un cuerpo de error mínimo debe dar al consumidor un código estable, un mensaje
apto para personas y detalles opcionales que localicen el problema. El código
es la clave para automatizar; el mensaje explica contexto; los detalles evitan
que el cliente tenga que volver a implementar validaciones del proveedor.

Los detalles deben identificar el campo o parte de la solicitud cuando eso sea
seguro y útil. No deben exponer secretos, reglas internas sensibles ni una
traza. Una referencia de correlación puede ayudar a soporte sin hacer pública
la arquitectura del servidor.

## Validación antes del efecto

Validar antes de producir un efecto permite que el consumidor corrija una
solicitud sin preguntarse si algo se aplicó parcialmente. Cuando una operación
sí admite efectos parciales o asíncronos, el contrato debe declararlo y ofrecer
una forma de observar el resultado, como se estudió con `202 Accepted`.

La validación de formato responde si la solicitud puede interpretarse. La
validación de dominio responde si esa solicitud tiene sentido en el estado
actual del sistema. Ambas pueden necesitar códigos HTTP distintos, pero deben
compartir una estructura que el consumidor pueda recorrer de manera uniforme.

## Paginación y continuidad

Una página necesita elementos, un límite solicitado o aplicado y una
continuación que el consumidor pueda usar sin comprender cómo se almacenan los
datos. Para que un cursor sea seguro, el contrato define un orden estable y
qué ocurre cuando la colección cambia durante el recorrido.

Un cursor puede codificar una posición, una marca de tiempo o una clave de
orden. El consumidor no debe interpretarlo ni construirlo. Si se invalida por
caducidad, permisos o cambio de consulta, la API debe devolver un error
accionable en lugar de repetir o saltar elementos en silencio.

## Invariantes

- Cada error expone un código estable y una acción comprensible.
- Los detalles de validación localizan una corrección sin filtrar secretos.
- La validación distingue una entrada mal formada de una regla de dominio.
- Una solicitud rechazada antes del efecto no deja cambios parcialmente
  aplicados.
- Una página declara un orden estable antes de entregar un cursor.
- Un cursor es opaco para el consumidor y específico de su consulta.
- Un límite máximo protege al proveedor sin convertir la respuesta en una
  sorpresa silenciosa.

## Preguntas de diseño

1. ¿Qué debe poder hacer un consumidor distinto después de cada código de
   error?
2. ¿Qué campo o regla conviene revelar para corregir una solicitud?
3. ¿Qué información interna sería peligrosa incluir en un detalle de error?
4. ¿Cuál es el orden estable que sostiene una continuación de página?
5. ¿Qué ocurre cuando un cursor ya no es válido para la consulta original?

## Siguiente paso

El modelo Rust representará un error con detalles seguros y una página con un
cursor opaco, límite validado y orden declarado. No construirá serialización ni
base de datos; hará visibles las invariantes que el consumidor necesita para
recuperarse y recorrer resultados.

## Decisiones registradas

- Los errores se enseñan como datos de decisión para consumidores, no como
  mensajes libres del servidor.
- La paginación se enseña desde la continuidad y el orden, no desde un offset
  elegido por costumbre.
- El capítulo permanece en `draft`; no está revisado ni publicado.
