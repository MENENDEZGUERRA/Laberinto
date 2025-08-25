# Laberinto

## 🩸 The Binding of Isaac: Cutre Edition — Raycaster en Rust

Un mini “Wolfenstein-like” hecho en Rust: un ray caster que renderiza un nivel completo y jugable, con minimapa, texturas de paredes y piso, música/SFX, pantalla de bienvenida y de victoria.
Tema libre inspirado en The Binding of Isaac (fan tribute, sin ánimo de lucro).

## ✨ Características

🎮 Movimiento: avanzar/retroceder, strafe y rotación (teclas y ratón horizontal).

🧱 Paredes con texturas por tipo de tile.

🧵 Piso texturizado (floor.png) y cielo negro.

🧭 Minimapa en esquina con posición del jugador.

🖥️ HUD con FPS.

🟦 Pantalla de bienvenida y ✅ pantalla de éxito (al tocar el tile de salida).

🔊 Música de fondo + efectos (OGG/WAV en assets/audio/).

🧟 Tema: The Binding of Isaac: Cutre Edition (estética simple + cutre vibes 😅).

## 🧰 Tecnologías

🦀 Rust (stable)

🧩 winit (ventana + input)

🧪 pixels (framebuffer en GPU)

📐 glam (vectores/rotaciones 2D)

🖼️ image (carga de PNG)

🔊 rodio (audio) (a través de audio.rs)

## 🕹️ Controles

ENTER — Empezar (desde la pantalla de bienvenida) / Reiniciar (desde victoria)

W/S — Avanzar / Retroceder

A/D — Strafe izquierda / derecha

← / → — Rotar

ESC — Capturar/soltar ratón (rotación horizontal con el mouse)

1 / 2 — Cambiar entre niveles de ejemplo

R — Reiniciar al spawn