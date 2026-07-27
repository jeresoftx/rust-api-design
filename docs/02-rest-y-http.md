# REST y semántica HTTP

**Estado:** draft

## Introducción

HTTP no es una tubería neutral para mover JSON. Sus métodos, códigos de estado,
cabeceras y reglas de caché forman parte del contrato que un consumidor usa
para decidir si puede reintentar, mostrar un resultado, corregir una entrada o
esperar una transición.

REST aprovecha esa semántica para modelar capacidades alrededor de recursos.
No convierte cualquier URL en REST por usar `GET` y `POST`: exige que los
nombres, métodos y respuestas conserven significado observable para quien
integra.

## Concepto

Un recurso es una representación identificable de algo que importa al dominio:
un pedido, una factura, una colección de productos o una operación en curso.
La URI identifica el recurso o la colección; el método HTTP expresa la clase
de interacción; la respuesta comunica el resultado dentro del protocolo.

El diseño nace de la capacidad del consumidor, no de la estructura de una
tabla. Por ejemplo, `GET /pedidos/42` expresa la consulta de un pedido. El
consumidor puede interpretar `200`, `404` y las cabeceras de caché sin conocer
el servicio, repositorio o consulta que produjo la respuesta.

Las piezas mínimas de esta semántica son:

- **recurso:** entidad o colección con significado de dominio;
- **método:** intención protocolaria, como leer, crear o reemplazar;
- **estado:** resultado observable de la interacción;
- **representación:** datos y metadatos que cruzan la frontera;
- **propiedad de reintento:** qué ocurre si una solicitud llega más de una vez.

## Problema

Cuando HTTP se trata como una envoltura de RPC, aparecen rutas con verbos
internos, métodos que cambian estado al leer y respuestas que siempre devuelven
`200` aunque el consumidor no pueda actuar correctamente. El costo no es solo
estético: clientes, proxies, cachés, herramientas de observabilidad y personas
que integran pierden señales que el protocolo ya ofrece.

El caso más delicado ocurre ante fallas parciales. Si el cliente no sabe si una
solicitud llegó al servidor, necesita entender si puede reintentarse. Sin una
semántica clara, puede duplicar un pago, crear dos pedidos o abandonar una
operación que sí se completó.

La solución tampoco es memorizar una tabla de códigos. Un `404` correcto con
un recurso mal elegido sigue siendo un contrato débil. El problema se resuelve
cuando recurso, método, estado y reintento cuentan la misma historia.

## Alternativas

La primera alternativa es ignorar HTTP y exponer acciones como
`POST /crearPedido` o `POST /cancelarPedido`. Puede ser directa para el equipo
que conoce la implementación, pero obliga al consumidor a aprender un
vocabulario paralelo y deja ambiguas propiedades como seguridad, caché e
idempotencia.

La segunda es aplicar reglas REST de forma mecánica. Por ejemplo, usar `PUT`
siempre que exista un identificador o devolver `204` para cualquier éxito. Esto
parece uniforme, pero puede ocultar la diferencia entre crear, reemplazar,
aceptar trabajo asíncrono o devolver una representación útil.

La tercera es elegir el estilo por la operación real: nombrar el recurso,
seleccionar el método por su semántica y devolver un estado que permita al
consumidor decidir qué hacer. Este curso adopta esa alternativa porque preserva
la capacidad de evolucionar y aprovechar el protocolo sin fingir que HTTP
resuelve todas las necesidades de dominio.

## Semántica de métodos

`GET` recupera una representación y debe ser seguro: observarlo no debe causar
un cambio de negocio. `POST` solicita procesamiento bajo la colección o un
recurso; puede crear una entidad, iniciar una operación o delegar una acción
que no encaja como reemplazo. `PUT` coloca una representación completa en una
URI conocida y debe ser idempotente. `PATCH` describe una modificación parcial
y necesita definir con precisión qué ocurre al repetirla. `DELETE` solicita la
eliminación o retirada de una representación y también debe definir la
semántica de repeticiones.

Idempotencia no significa que la primera y segunda respuesta sean idénticas.
Significa que repetir la misma solicitud con la misma intención deja el
recurso en un estado equivalente. Esta propiedad permite que un consumidor
reintente después de una falla de red sin convertir la incertidumbre en un
efecto duplicado.

## Semántica de estados

Los códigos de estado no reemplazan un cuerpo de error útil, pero dan la
primera clasificación protocolaria. `200` representa una respuesta exitosa
con contenido; `201` comunica creación e idealmente identifica el recurso;
`202` acepta trabajo que todavía no terminó; `204` confirma éxito sin cuerpo.

En el lado de los errores, `400` señala una solicitud que no puede
interpretarse, `401` una falta de autenticación, `403` una autorización
insuficiente, `404` un recurso no disponible para ese contrato, `409` un
conflicto con el estado actual y `422` una solicitud entendida pero inválida
según reglas de dominio. La elección exige explicar qué puede hacer el
consumidor después, no solo clasificar la falla.

## Invariantes

- Una URI nombra un recurso o una colección, no una función interna.
- Un método conserva su semántica de seguridad e idempotencia declarada.
- Un estado HTTP permite la primera decisión del consumidor sobre el resultado.
- Un error de dominio incluye información accionable además del código HTTP.
- Un reintento no debe duplicar un efecto cuando la operación se declara
  idempotente.
- Una respuesta asíncrona declara cómo observar el resultado posterior.
- Caché, concurrencia y permisos se vuelven parte del contrato cuando afectan
  el comportamiento observable.

## Preguntas de diseño

1. ¿Qué recurso necesita observar o modificar el consumidor?
2. ¿El método elegido expresa lectura, creación, reemplazo o modificación
   parcial de manera honesta?
3. ¿Puede el consumidor reintentar después de una falla de red? ¿Por qué?
4. ¿Qué estado y qué cuerpo le permiten distinguir una corrección de una
   espera o un abandono?
5. ¿Qué parte de la representación es estable y qué parte puede evolucionar?

## Siguiente paso

El modelo Rust del capítulo representará una solicitud HTTP como intención,
propiedad de reintento y resultado observable. No implementará un servidor:
hará visible cuándo una combinación de método y estado contradice el contrato
que pretende comunicar.

## Decisiones registradas

- REST se enseña como semántica de recursos sobre HTTP, no como una convención
  de rutas con JSON.
- La idempotencia se explica desde el efecto observable de reintentos.
- El capítulo permanece en `draft`; no está revisado ni publicado.
