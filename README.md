# Pomodoro TUI

Aplicación de temporizador Pomodoro con interfaz de terminal (TUI) escrita en Rust.

## Características

- Interfaz de terminal interactiva con ratatui
- Notificaciones del sistema al completar cada fase
- Configuración personalizable (tiempos de trabajo, descanso corto y largo)
- Controles simples de teclado

## Requisitos

- Rust 1.75 o superior
- Sistema operativo Linux (para notificaciones)
- Dependencias del sistema:
  - Linux: `libdbus` (para notificaciones)

## Instalación desde código fuente

### 1. Clonar el repositorio

```bash
git clone https://github.com/tu-usuario/pomodoro-rust.git
cd pomodoro-rust
```

### 2. Instalar dependencias del sistema

**Linux (Debian/Ubuntu):**

```bash
sudo apt install libdbus-1-dev
```

**Linux (Arch Linux):**

```bash
sudo pacman -S dbus
```

**Linux (Fedora):**

```bash
sudo dnf install dbus-devel
```

### 3. Compilar el proyecto

```bash
cargo build --release
```

### 4. Ejecutar

```bash
cargo run --release
```

O ejecutar el binario compilado directamente:

```bash
./target/release/pomodoro-rust
```

## Configuración

La configuración se guarda en `~/.config/pomodoro-rust/config.toml`

Puedes editar este archivo para personalizar:

- `work_duration`: Duración del trabajo (default: 25 minutos)
- `short_break_duration`: Descanso corto (default: 5 minutos)
- `long_break_duration`: Descanso largo (default: 15 minutos)
- `sessions_before_long_break`: Sesiones antes del descanso largo (default: 4)

## Controles

| Tecla | Acción |
|-------|--------|
| `Espacio` | Iniciar / Pausar |
| `s` | Saltar a la siguiente fase |
| `r` | Reiniciar el temporizador |
| `q` | Salir |

## Estructura del proyecto

```
pomodoro-rust/
├── src/
│   ├── main.rs      # Punto de entrada
│   ├── config.rs    # Gestión de configuración
│   ├── timer.rs     # Lógica del temporizador
│   ├── ui.rs        # Interfaz de usuario
│   └── notify.rs    # Notificaciones del sistema
├── Cargo.toml       # Dependencias del proyecto
└── README.md        # Este archivo
```
