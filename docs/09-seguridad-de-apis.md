# Seguridad de aplicaciones y OWASP

**Estado:** draft

## Introducción

La seguridad de una API no aparece al final como una lista de cabeceras. Cada
contrato acepta entradas, revela salidas y concede capacidades; por ello cada
decisión de diseño puede ampliar o reducir una frontera de ataque.

Este capítulo estudia amenazas comunes de APIs con el criterio de ingeniería:
identificar el activo, la frontera, el abuso posible y la evidencia que permite
detectarlo. OWASP aporta un lenguaje de riesgos, no un sustituto del análisis
del sistema propio.

## Concepto

Validar entradas confirma forma, tamaño, rango y significado antes de que una
solicitud alcance una operación sensible. Autorizar protege la capacidad y el
recurso, mientras que la exposición controlada evita devolver campos, errores
o metadatos que el consumidor no necesita.

La seguridad también incluye límites contra automatización abusiva, secretos
fuera del código y registros que permitan investigar sin conservar tokens o
datos personales innecesarios. Una defensa útil conserva una decisión
observable: rechazar, limitar, ocultar o alertar por una razón explícita.

## Problema

Confiar en el cliente para validar, ocultar un botón como única autorización o
devolver errores internos convierte detalles de implementación en fronteras
frágiles. Una ruta puede parecer protegida y aun así permitir enumerar
recursos, exceder límites o acceder a datos de otra organización.

Aplicar controles genéricos sin clasificar el riesgo también falla. Un límite
que protege una búsqueda puede bloquear una operación legítima; un log muy
detallado puede crear una nueva fuga; una validación sintáctica no prueba que
la acción tenga sentido para el recurso solicitado.

## Alternativas

La primera alternativa es añadir controles al detectar un incidente. Reacciona
tarde y deja cada capacidad con reglas inconsistentes.

La segunda es copiar una lista de controles sin vincularla a activos ni flujos.
Puede producir muchas configuraciones y poca evidencia de protección real.

La tercera es modelar cada frontera por entrada, autorización, exposición y
abuso, revisar amenazas antes de publicar y registrar señales seguras. Este
curso adopta la tercera alternativa.

## Invariantes

- La validación ocurre del lado del proveedor antes de ejecutar la capacidad.
- La autorización protege acción y recurso, no solo visibilidad de interfaz.
- Las respuestas y errores exponen solo datos necesarios para la decisión.
- Los secretos nunca viajan a logs, ejemplos ni mensajes de error.
- Los límites contra abuso tienen una señal recuperable y son observables.
- Los hallazgos de seguridad se corrigen con evidencia y regresiones cubiertas.

## Preguntas de diseño

1. ¿Qué activo protege esta operación y quién no debe conocerlo?
2. ¿Qué entrada necesita validación estructural y cuál validación de dominio?
3. ¿Qué respuesta permitiría enumerar recursos o permisos ajenos?
4. ¿Qué señal diferenciaría uso legítimo de abuso automatizado?
5. ¿Qué evidencia permite investigar un incidente sin registrar un secreto?

## Siguiente paso

El modelo Rust del capítulo representará una frontera de API por entrada,
sensibilidad y exposición de error. No implementará criptografía ni un WAF;
hace visible qué combinación de datos y respuesta debe rechazarse.

## Decisiones registradas

- OWASP se usa como lenguaje para razonar riesgos, no como lista mecánica.
- Validación, autorización y exposición forman una misma frontera de seguridad.
- El capítulo permanece en `draft`; no está revisado ni publicado.
