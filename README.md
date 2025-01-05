# API de Gestión Veterinaria en Rust

## Sección Técnica

### Descripción
API REST desarrollada en Rust para la gestión de clínicas veterinarias, permitiendo el manejo de clínicas, clientes, mascotas e historias clínicas.

### Dependencias Principales
- `rocket`: Framework web para Rust
- `serde`: Serialización/deserialización de datos
- `uuid`: Generación de identificadores únicos
- `chrono`: Manejo de fechas
- `rocket_cors`: Soporte para CORS
- `log` y `env_logger`: Sistema de logging

### Arquitectura
El proyecto sigue una arquitectura en capas:

1. **Controllers**: Manejo de endpoints HTTP y DTOs
   - Clínicas
   - Clientes
   - Mascotas
   - Historias Clínicas

2. **Services**: Lógica de negocio
   - Validaciones
   - Transformación de datos
   - Coordinación entre repositorios

3. **Repositories**: Persistencia de datos
   - Implementación en memoria
   - Interfaces genéricas para futura extensibilidad

4. **Models**: Entidades del dominio
   - Clínica
   - Cliente
   - Mascota
   - Historia Clínica
   - Entrada de Historia Clínica

## Sección Didáctica

### Características de Rust Utilizadas

#### Gestión de Concurrencia
- **Mutex**: Utilizado para el acceso seguro a los servicios compartidos
```rust:src/main.rs
rocket::build()
    .attach(make_cors())
    .manage(Mutex::new(clinica_service))
    .manage(Mutex::new(cliente_service))
    .manage(Mutex::new(mascota_service))
    .manage(Mutex::new(historia_clinica_service))
```

#### Traits y Genéricos
- Uso de traits para definir comportamientos de repositorios
- Implementaciones genéricas en servicios para permitir diferentes tipos de repositorios
```rust:src/services/cliente_service.rs
pub struct ClienteService<T: ClienteRepository> {
    repository: T,
}

impl<T: ClienteRepository> ClienteService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}
```

#### Pattern Matching
- Manejo de errores mediante `Result` y `Option`
- Match expressions para control de flujo
```rust:src/controllers/cliente_controller.rs
match result {
    Ok(cliente) => Ok(Json(cliente)),
    Err(_) => Err(Status::NotFound),
}
```

#### Ownership y Borrowing
- Clonación controlada de datos
- Referencias compartidas para lectura
- Referencias mutables para escritura
```rust:src/services/clinica_service.rs
pub fn actualizar_clinica(
    &mut self,
    id: Uuid,
    nombre: String,
    direccion: String,
    telefono: String,
    correo: String,
) -> Result<Clinica, String> {
    let clinica = self.repository.obtener(id)
        .ok_or_else(|| "La clínica no existe".to_string())?;

    let clinica_actualizada = Clinica {
        id: clinica.id,
        nombre,
        direccion,
        telefono,
        correo,
    };

    self.repository.guardar(clinica_actualizada.clone())?;
    Ok(clinica_actualizada)
}
```

#### Macros y Atributos
- Macros de Rocket para definir rutas
- Atributos para derivar traits
```rust:src/controllers/clinica_controller.rs
#[derive(Debug, Deserialize)]
pub struct ClinicaCreateDto {
    pub nombre: String,
    pub direccion: String,
    pub telefono: String,
    pub correo: String,
}

#[put("/clinicas/<id>", data = "<clinica_dto>")]
pub async fn actualizar_clinica(
    id: &str,
    clinica_dto: Json<ClinicaCreateDto>,
    service: &State<ClinicaServiceType>
) -> Result<Json<Clinica>, Status>
```

#### Error Handling
- Uso de `Result` para manejo de errores
- Propagación de errores con el operador `?`
- Conversión de errores entre tipos
```rust:src/controllers/cliente_controller.rs
let uuid = Uuid::parse_str(&id)
    .map_err(|err| {
        error!("Error parsing UUID '{}': {}", id, err);
        Status::BadRequest
    })?;

service.lock()
    .map_err(|_| Status::InternalServerError)?
    .obtener_cliente(uuid)
    .map(|cliente| Json(cliente.clone()))
    .ok_or(Status::NotFound)
```

### Funcionalidades Destacadas

1. **CORS Configurado**
   - Soporte para múltiples orígenes
   - Métodos HTTP permitidos configurables
   - Headers personalizables

2. **Logging Integrado**
   - Diferentes niveles de log
   - Formato personalizable
   - Útil para debugging y monitoreo

3. **DTOs y Validación**
   - Separación entre modelos de dominio y DTOs
   - Validación de UUIDs y datos de entrada
   - Transformación controlada de datos

4. **Gestión de Estado**
   - Estado compartido thread-safe
   - Acceso controlado mediante Mutex
   - Manejo de errores de concurrencia

### Declaración de Endpoints y DTOs

#### Anotaciones (Attributes)
Las anotaciones en Rust son metadatos que se aplican a módulos, tipos o funciones. Algunas importantes son:

1. **Derive Attributes**:
```rust:src/controllers/clinica_controller.rs
#[derive(Debug, Deserialize)]
pub struct ClinicaCreateDto {
    pub nombre: String,
    pub direccion: String,
    pub telefono: String,
    pub correo: String,
}
```
- `#[derive(Debug)]`: Implementa automáticamente la capacidad de depuración
- `#[derive(Deserialize)]`: Permite deserializar JSON a estructuras Rust
- Estas anotaciones son macros que generan código automáticamente

2. **Endpoint Attributes**:
```rust:src/controllers/clinica_controller.rs
#[put("/clinicas/<id>", data = "<clinica_dto>")]
pub async fn actualizar_clinica(
    id: &str,
    clinica_dto: Json<ClinicaCreateDto>,
    service: &State<ClinicaServiceType>
) -> Result<Json<Clinica>, Status>
```
- `#[put("/clinicas/<id>")]`: Define la ruta y método HTTP
- `data = "<clinica_dto>"`: Indica que el endpoint espera datos en el body

#### Estructura de un Endpoint
Cada endpoint sigue un patrón común:

1. **Declaración de Ruta**:
```rust
#[post("/ruta/<parametro>")]
```

2. **Función Asíncrona**:
```rust
pub async fn nombre_funcion(
    parametro: TipoParametro,
    dto: Json<TipoDto>,
    service: &State<TipoService>
) -> Result<Json<TipoRespuesta>, Status>
```

3. **Manejo de Estado**:
- Acceso al servicio mediante `&State<T>`
- Uso de Mutex para acceso seguro
```rust:src/controllers/cliente_controller.rs
#[get("/clientes")]
pub async fn listar_clientes(service: &State<ClienteServiceType>) -> Result<Json<Vec<Cliente>>, Status> {
    let clientes = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .listar_clientes()
        .into_iter()
        .cloned()
        .collect();
    Ok(Json(clientes))
}
```

4. **Respuestas**:
- Uso de `Result` para manejar éxito/error
- `Json<T>` para serializar respuestas
- `Status` para códigos HTTP
```rust:src/controllers/cliente_controller.rs
match result {
    Ok(cliente) => Ok(Json(cliente)),
    Err(_) => Err(Status::NotFound),
}
```

#### DTOs (Data Transfer Objects)
Los DTOs son estructuras que definen el formato de datos para las peticiones:

1. **Definición**:
```rust:src/controllers/mascota_controller.rs
#[derive(Debug, Deserialize)]
pub struct MascotaCreateDto {
    pub nombre: String,
    pub especie: String,
    pub raza: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub id_cliente: String,
}
```

2. **Uso**:
- Validación de datos de entrada
- Transformación a modelos de dominio
- Separación entre API y lógica de negocio
