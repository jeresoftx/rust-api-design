# GraphQL y gRPC

**Estado:** draft

## Introducción

REST no es el único estilo para ofrecer una capacidad de negocio. GraphQL
permite que un consumidor declare la forma de los datos que necesita; gRPC
prioriza contratos tipados y llamadas de bajo costo entre procesos. Ambos
pueden resolver problemas reales, pero también pueden esconder límites,
errores y costos si se adoptan como una preferencia de herramienta.

Este capítulo compara GraphQL y gRPC como contratos para consumidores. No
busca convertir cada API a un estilo nuevo: busca reconocer qué interacción
necesita el sistema, qué promesa puede sostener el proveedor y qué costo
operativo acepta el equipo.

## Concepto

GraphQL expone un grafo tipado que el consumidor consulta declarando campos y
relaciones. Su contrato describe tipos, operaciones y mutaciones; el servidor
resuelve la selección solicitada. Esa flexibilidad puede evitar respuestas con
datos irrelevantes, pero exige controlar profundidad, complejidad, autorización
por campo y el patrón de acceso a datos.

gRPC define servicios y mensajes tipados, normalmente con Protocol Buffers. Su
contrato favorece clientes generados, llamadas entre servicios y streaming
cuando la interacción lo requiere. La eficiencia del transporte no elimina la
necesidad de modelar errores, compatibilidad, límites y observabilidad para
quien integra el servicio.

## Problema

Adoptar GraphQL para reemplazar cualquier colección de endpoints puede crear
consultas costosas o contratos que filtran detalles internos del grafo. Adoptar
gRPC solo por desempeño puede dificultar la integración de navegadores,
proxies o consumidores externos que necesitan interfaces HTTP ampliamente
interoperables.

El error común es comparar sintaxis: rutas frente a queries, JSON frente a
mensajes binarios. La decisión útil compara el trabajo que el consumidor debe
hacer, el control que necesita sobre los datos, la estabilidad del contrato y
la capacidad operativa del proveedor para sostenerlo.

## Alternativas

La primera alternativa es estandarizar un solo estilo para toda interacción.
Simplifica la plataforma al inicio, pero fuerza problemas distintos a compartir
las mismas limitaciones.

La segunda es adoptar GraphQL o gRPC por moda, rendimiento teórico o porque un
equipo ya conoce su ecosistema. Puede producir una implementación rápida sin
una promesa clara para consumidores ni una estrategia de operación.

La tercera es elegir el estilo por la forma de interacción: GraphQL cuando el
consumidor necesita componer vistas desde un grafo controlado; gRPC cuando
servicios con contrato compartido necesitan llamadas tipadas o streaming;
REST cuando el recurso HTTP y su interoperabilidad siguen expresando mejor la
capacidad. Este curso adopta la tercera alternativa.

## Límites de cada contrato

Una consulta GraphQL debe tener límites de profundidad, complejidad y tamaño;
la posibilidad de seleccionar campos no autoriza trabajo ilimitado. Los errores
deben permitir al consumidor entender qué parte de su solicitud falló sin
filtrar detalles internos. Los resolvers también requieren observabilidad para
detectar fan-out, consultas repetidas y dependencias lentas.

Un servicio gRPC debe versionar mensajes de forma compatible, documentar sus
códigos de error y declarar si una llamada puede repetirse. El streaming
necesita límites de duración, cancelación, presión inversa y observabilidad.
Un mensaje tipado no vuelve segura una evolución incompatible ni convierte un
servicio interno en una interfaz adecuada para todo consumidor.

## Invariantes

- El estilo se elige por la interacción y el consumidor, no por preferencia de
  framework.
- Un contrato GraphQL limita la forma y el costo de las consultas observables.
- Un contrato gRPC conserva compatibilidad de mensajes y semántica de errores.
- La autorización se aplica a la capacidad y los datos, no solo a la conexión.
- La generación de clientes acelera integración, pero no decide el diseño.
- Métricas y trazas permiten observar el costo real de resolvers y llamadas.
- Un estilo nuevo no elude las reglas de evolución del capítulo anterior.

## Preguntas de diseño

1. ¿Qué decisión del consumidor requiere componer datos y cuál requiere una
   llamada tipada entre servicios?
2. ¿Qué límite impide que una consulta GraphQL convierta flexibilidad en carga
   impredecible?
3. ¿Qué error o metadato necesita un cliente gRPC para reaccionar sin adivinar?
4. ¿Qué consumidores perderían interoperabilidad si una capacidad dejara HTTP?
5. ¿Cómo detectaríamos una dependencia lenta detrás de un resolver o stream?

## Siguiente paso

El modelo Rust del capítulo representará una elección de estilo con la
interacción que la justifica y los límites que debe sostener. No implementará
un servidor GraphQL ni gRPC: hará explícito que el transporte no sustituye un
contrato observable.

## Decisiones registradas

- GraphQL y gRPC se enseñan como alternativas condicionadas por la interacción,
  no como reemplazos automáticos de REST.
- Los límites de consulta, compatibilidad y operación son parte del contrato.
- El capítulo permanece en `draft`; no está revisado ni publicado.
