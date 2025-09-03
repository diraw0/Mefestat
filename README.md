# Mefestat

**Descripción**  
Mefestat es una aplicación para Windows que permite ejecutar programas específicos a través de la red Tor, asegurando que solo el tráfico de esa aplicación pase por Tor.

**Requisitos**  
- Windows 10 o superior.  
- Rust 2021 (para compilar desde código fuente).  
- `tor.exe` incluido en `assets/Tor/`.  
- WinDivert instalado (para redirección de tráfico).  

**Instalación**  
1. Clonar el repositorio:  
   git clone https://github.com/diraw0/Mefestat
   cd Mefestat

2. Compilar el proyecto:

   cargo build --release
3. Ejecutar el binario generado en `target/release/mefestat.exe`.

**Uso**

1. Abrir la aplicación.
2. Seleccionar un archivo `.exe`.
3. Presionar “Lanzar por Tor”.
4. La aplicación se ejecutará con su tráfico redirigido a través de Tor.

**Notas**

* Asegúrate de que `assets/Tor/tor.exe` esté presente.
* WinDivert debe estar instalado y cargado para que la redirección funcione correctamente.

