use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoriaClinica {
    pub id: Uuid,
    pub id_mascota: Uuid,
    pub id_cliente: Uuid,
    pub fecha_creacion: DateTime<Utc>,
    pub fecha_actualizacion: DateTime<Utc>,
}

impl HistoriaClinica {
    pub fn new(id_mascota: Uuid, id_cliente: Uuid) -> Self {
        let ahora = Utc::now();
        Self {
            id: Uuid::new_v4(),
            id_mascota,
            id_cliente,
            fecha_creacion: ahora,
            fecha_actualizacion: ahora,
        }
    }
}
