# TETRINET-REBORN ROADMAP

## P1. Project basic setup

-  (OK) R1. Set up a Rust Workspace

-  (OK) R2. Add Bevy 0.16 as a dependency

-  (OK) R3. Basic skeleton : main loop & basic state machine : MainMenu, Connecting, InGame

## P2. Networking Layer

### P2.1. Define the TetriNET Protocol 

- R4. Enumerate messages that the client receives (e.g. `playerjoin`, `pline`, `field`, ...)

- R5. Enumerate messages that the client sends (e.g. `team`, `pline`, ...)

- R6. Parse incoming strings into Rust enums/structs

- R7. Serialize messages into TetriNET-compliant strings

### P2.2. Networking System

- R8. TCP client connection

- R9. Async network task that reads incoming messages, and pushes them into Bevy via events or channels

- R10. System that listens to player actions and emits messages to the server

## P3. Game State & ECS Modeling

### P3.1. Design the ECS structure

- R11. Define the entities (e.g., local player, remote players, the playing field, blocks)
- R12. Specify components they need? (e.g., `Position`, `Shape`, `PlayerId`, `Board`)
- R13. Specify global resources (e.g., game timer, scoreboards)

### P3.2. Systems

- R14. Spawning systems (e.g., initialize board, new piece spawns)
- R15. Input handling
- R16. Piece movement and rotation
- R17. Line clearing
- R18. Attack/defense mechanics

## P4. Rendering and UI

### P4.1. Draw the playing field

- R19. Use Bevy’s 2D API to render blocks
- R20. Use a camera to show the whole board
- R21. Support multiple player fields

### P4.2. Game UI

- R22. Integrate `bevy_egui` or `bevy_ui`
- R23. Display player list, scores, current piece queue, special blocks, etc.
- R24. Menu UI (joining server, nickname input, etc.)

## P5. Input & Controls

### P5.1. Input Mapping

- R25. Move/rotate piece
- R26. Send messages / chat
- R27. Use special blocks

### P5.2. Command Handling

- R28. Handle in-game chat (`pline`)
- R29. Keybindings for item usage, targeting players, etc.

## P6. Game Logic

### P6.1 Piece System

- R30. Shapes, randomization (maybe 7-bag?)
- R31. Piece rotation rules (cf. "Super Rotation System (SRS))
- R32. Ghost piece (?)

### P6.2. Board Updates

- R33. Gravity, locking, line clears
- R34. Garbage lines from attacks
- R35. Special blocks behavior

## P7. Multiplayer Sync

### P7.1. Handle Remote Player Events

- R36. Parse `field` updates
- R37. Sync piece positions or garbage
- R38. Display other players' fields

### P7.2. **Latency Handling**

- R39. Queueing or interpolation if needed

## P8. Audio

- R40. Sound effects and background music

## P9. Improvements

- R41. Local play / bot players
- R42. Replay system
- R43. Spectator mode
- R44. Mobile port / WebAssembly export

