# **Desafío Levannta (Prueba Técnica) - Cálculo de MRR y Score**

Este proyecto implementa una aplicación web en **Rust** que permite calcular el **MRR** y el **Score** basado en una cartera de clientes de suscripción. La lógica del sistema sigue el enunciado del desafío, con ciertas interpretaciones ajustadas para manejar inconsistencias detectadas.

---

## **Instrucciones de Configuración**

### **1. Requisitos Previos**
- Tener instalado **Rust** y **Cargo** en tu máquina. Si no los tienes, puedes instalarlos siguiendo las instrucciones en [Rust Lang](https://www.rust-lang.org/tools/install).

- Instalar `cargo-watch`:
    ```bash
    cargo install cargo-watch

### **2. Clonar el Repositorio**
- Clonar repositorio en el sistema y ubicar en el proyecto:
    ```bash
    git clone https://github.com/JulioC35ar/desafio_levannta.git
    cd desafio_levannta

### **3. Ejecutar la Aplicación**
- Construir y ejecutar la aplicación:
    ```bash
        cargo run
- Por defecto, el servidor se ejecutará en http://127.0.0.1:8080.

---

## **Descripción Técnica**

### **Decisiones Tomadas**
1. **Cálculo del MRR**:
   - **Descripción**: Se calculó como el promedio mensual de ingresos para reflejar la estabilidad de ingresos recurrentes.
   - **Fórmula**:
      MRR = Ingresos Totales / Meses Activos

2. **Fórmula del Score**:
   - **Descripción**: Se utilizó la fórmula proporcionada en el enunciado.
   - **Fórmula**:
    Score = (MRR / 1000) - (Churn Rate * 10)
   - **Observaciones**:
     - La división del MRR por \(1000\) y el peso del Churn Rate (\( \times 10 \)) hacen que el score sea difícil de superar el umbral de 70.

3. **Condición para el Préstamo**:
   - **Descripción**: El préstamo máximo se otorga solo si el score supera 70, y su valor es igual al MRR.
   - **Criterios**:
      - Si `Score > 70`, entonces `Max Loan = MRR`.
      - Si `Score <= 70`, entonces `Max Loan = 0`.

4. **Endpoints REST**:
   - **Descripción**: Se implementaron los tres endpoints principales para la aplicación indicados en el enunciado.
   - **Endpoints**:
     - `POST /portfolio`: Carga la cartera de clientes.
     - `POST /apply-loan`: Solicita un adelanto basado en la cartera cargada.
     - `GET /loan-status`: Consulta el estado del préstamo solicitado.

## **Ejemplos de Solicitudes**

A continuación, se muestran ejemplos de cómo consumir los endpoints implementados utilizando `curl`.

---

### **1. Cargar Cartera de Clientes**
**Endpoint**: `POST /portfolio`

Este endpoint permite cargar un archivo CSV con la cartera de clientes.

#### **Ejemplo con `curl`**:
Ubicar documento a procesar y cambiar la ruta (ingresar_ruta)
```bash
curl -X POST http://127.0.0.1:8080/portafolio \
     -H "Content-Type: text/plain" \
     --data-binary @ingresar_ruta/portafolio.csv
```
#### **Archivo de ejemplo**:
Basado en el domento proporcionado en el enunciado del desafío: [Descargar archivo CSV de ejemplo](./cartera.csv)

### **2. Solicitar Adelanto**
  **Endpoint**: `POST /apply-loan`
  Este endpoint permite solicitar un adelanto basado en la cartera cargada.
  
  #### **Ejemplo con `curl`**:
    ```bash
    curl -X POST http://127.0.0.1:8080/apply-loan \
         -H "Content-Type: application/json" \
         -d '{"monto": 1000}'
    ```
### **3. Consultar Estado del Préstamo**
**Endpoint**: `GET /loan-status`
Este endpoint permite consultar el estado de la solicitud de adelanto.
  #### **Ejemplo con `curl`**:
      ```bash
        curl -X GET http://127.0.0.1:8080/loan-status
      ```

