# Terminal Emulator in Rust

Este proyecto es un emulador de terminal simple escrito en Rust. Su objetivo principal es servir como un recurso educativo para desarrolladores interesados en aprender sobre el funcionamiento interno de los emuladores de terminal en un entorno Linux.

## Estado del Proyecto

**Este es un trabajo en progreso.** La funcionalidad es limitada y se está desarrollando activamente. No se recomienda para uso diario, sino como una herramienta de aprendizaje y un ejemplo de código.

## Características

*   **Intérprete de Secuencias de Escape VT100:** (En desarrollo) Parsea e interpreta secuencias de escape para el movimiento del cursor, colores y otros atributos de texto.
*   **Integración con Pseudoterminales (PTY):** Se comunica con un proceso PTY para ejecutar un shell (como `bash`) y manejar la entrada/salida.
*   **Renderizado con `crossterm`:** Utiliza el crate `crossterm` para la manipulación del terminal anfitrión.

## Requisitos

*   [Rust](https://www.rust-lang.org/tools/install) (Edición 2024 o posterior)
*   Un sistema operativo tipo Unix (Linux, macOS)

## Cómo Empezar

1.  **Clona el repositorio:**
    ```sh
    git clone https://github.com/jcmt2k/terminal-emulator.git
    cd terminal-emulator
    ```

2.  **Compila el proyecto:**
    ```sh
    cargo build
    ```

3.  **Ejecuta el emulador de terminal:**
    ```sh
    cargo run
    ```

## Contribuciones

Las contribuciones son bienvenidas. Si deseas mejorar este proyecto, por favor lee la guía de [CONTRIBUTING.md](CONTRIBUTING.md) para más detalles sobre cómo puedes ayudar.

## Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más detalles.
