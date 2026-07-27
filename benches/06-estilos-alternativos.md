# Decisión de benchmark: estilos alternativos

**Estado:** draft

No se agrega benchmark ejecutable. El modelo selecciona un estilo y valida un
límite en memoria; medirlo no decidiría si una vista requiere GraphQL, una
integración requiere gRPC ni si un recurso debe permanecer interoperable por
HTTP.

Un benchmark será pertinente cuando una integración concreta plantee una
decisión reproducible, como comparar latencia de resolvers bajo un presupuesto
fijo, serialización de mensajes en una llamada gRPC o comportamiento de presión
inversa durante un stream. Esa medición deberá nombrar la decisión de
ingeniería que puede modificar.
