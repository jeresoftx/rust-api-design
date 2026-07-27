# Decisión de benchmark: experiencia del consumidor

**Estado:** draft

No se agrega benchmark ejecutable. `ApiError` y `Page` son modelos en memoria
que no representan serialización, validación sobre carga real ni acceso a una
colección persistente. Medir su construcción no respondería una pregunta que
pueda cambiar el diseño de la API.

Un benchmark será pertinente cuando exista una hipótesis reproducible sobre
serialización de errores, tamaño de páginas, validación por lote o consistencia
de una consulta. La métrica deberá estar ligada a esa decisión, no a una cifra
aislada.
