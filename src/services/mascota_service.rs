use crate::models::Mascota;
use crate::repositories::mascota_repository::MascotaRepository;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct MascotaService<T: MascotaRepository> {
    repository: T,
}

impl<T: MascotaRepository> MascotaService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn crear_mascota(
        &mut self,
        nombre: String,
        especie: String,
        raza: String,
        fecha_nacimiento: Option<NaiveDate>,
        id_cliente: Uuid,
    ) -> Result<Mascota, String> {
        let mascota = Mascota::new(nombre, especie, raza, fecha_nacimiento, id_cliente);
        self.repository.guardar(mascota.clone())?;
        Ok(mascota)
    }

    pub fn obtener_mascota(&self, id: Uuid) -> Option<&Mascota> {
        self.repository.obtener(id)
    }

    pub fn listar_mascotas(&self) -> Vec<&Mascota> {
        self.repository.listar()
    }

    pub fn listar_mascotas_cliente(&self, id_cliente: Uuid) -> Vec<&Mascota> {
        self.repository.listar_por_cliente(id_cliente)
    }
}
