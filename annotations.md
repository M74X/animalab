Exacto. No compila. El compilador exige que todas las ramas del if produzcan el mismo tipo.

  SIN esta regla →  x podría ser i32 o &str según runtime → el compilador no sabe cuánta memoria reservar
  CON esta regla →  el tipo de x se conoce en compilación → cero sorpresas en producción
  EL COSTO ES    →  todas las ramas deben retornar el mismo tipo — el compilador lo verifica

¿Puedes llenar tú esas tres líneas con tus propias palabras? No copies las mías.

sin una regla de tipado es dificil definir la reserva de memoria, en otros lenguajes como js eso convertiria tu
codigo a codigo spagueti, generando errores a futuro razon pro la cual mucha gente se queja den node

con una regla de tipado fuerte es mas facil para el programador seguir la secuencia de programacion y entender el errores

tendriamos que tener en claro que typo de variable vamos