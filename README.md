# 🎮 Game RPG Program 

Este es un programa desarrollado en **Solana** utilizando el framework **Anchor**.  
Permite a los usuarios crear su propio juego RPG y gestionar una lista de personajes directamente en la blockchain, incluyendo sistema de clases, estadísticas y subida de nivel.

---

## 🚀 Características

**Persistencia On-Chain:**  
Los datos se guardan en cuentas PDA (Program Derived Addresses) vinculadas a la wallet del jugador.

**Clases Disponibles:**  
El programa permite crear personajes con una clase específica:

- Guerrero  
- Mago  
- Arquero  
- Asesino  

Cada clase asigna estadísticas base automáticamente (vida, mana, fuerza, defensa, velocidad e inteligencia).

**Sistema de Experiencia y Nivel:**  

- Cada personaje inicia en nivel 1.  
- Cada 100 XP sube de nivel.  
- Al subir nivel aumenta su fuerza, defensa y vida automáticamente.  

**Gestión de Personajes (CRUD parcial):**

- Crear juego  
- Agregar personajes (máx. 5)  
- Ver personajes  
- Ganar experiencia  
- Eliminar personaje  

**Seguridad:**  
Validación de propiedad (Owner) para asegurar que solo tú puedas modificar tu juego.

---

## 🛠️ Tecnologías utilizadas

- Rust  
- Anchor Framework  
- Solana Web3.js   
- Solana Playground 
- Solana Blockchain  

---

## 📦 Estructura del Programa

### Instrucciones (Methods)

| Método | Descripción |
|--------|------------|
| `crear_juego` | Inicializa la cuenta del juego con el nombre y owner. |
| `agregar_personaje` | Añade un personaje con estadísticas según su clase (Máx. 5). |
| `ver_personajes` | Muestra en logs la lista de personajes. |
| `ganar_experiencia` | Agrega XP y gestiona la subida de nivel automática. |
| `eliminar_personaje` | Elimina un personaje por nombre. |

---

## 🔧 Configuración y Despliegue

### 1️⃣ Clonar el repositorio

```bash
git clone https://github.com/Magi1610/gameRPG-Solana.git
```

### 2️⃣ Abrir en Solana Playground

- Subir los archivos al Playground.  
- Cambiar el `declare_id!` en `lib.rs` por tu Program ID generado.  

### 3️⃣ Build & Deploy

En la terminal de Playground:

```bash
build
deploy
```

---

## ⚠️ Errores Personalizados

El programa incluye un sistema de manejo de errores (`error_code`):

- `CampoVacio` → Campos vacíos o con espacios.  
- `NoEresElOwner` → Intento de modificación por una wallet ajena.  
- `PersonajeNoEncontrado` → El personaje no existe.  
- `EspacioInsuficiente` → Límite máximo de personajes alcanzado.  

---

Creado con por Irving Martínez
