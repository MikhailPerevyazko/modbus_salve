use serde::Deserialize;
use std::{error::Error, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct ModbusRegistersMap {
    pub coils: Vec<Coils>,
}
#[derive(Debug, Deserialize)]
pub struct Coils {
    pub parameters_name: String,
    pub unit_id: u8,
    pub start_address: u16,
    pub type_storage: i32,
    pub parameters_type: String,
}
#[derive(Debug)]
pub struct YamlCoilsPath {
    pub file_path: PathBuf,
}

impl YamlCoilsPath {
    fn load_from_yaml(&self) -> Result<ModbusRegistersMap, Box<dyn Error>> {
        let handler = std::fs::File::open(&self.file_path).unwrap();
        let data: ModbusRegistersMap = serde_yaml::from_reader(&handler).unwrap();
        Ok(data)
    }
}

pub fn read_yaml_coils() -> Vec<Coils> {
    let path_to_coils_yaml: PathBuf =
        PathBuf::from("/home/Mikhail/projects/try_to_modbus/modbus_tcp/modbus_registers.yaml");
    let yaml_coils = YamlCoilsPath {
        file_path: path_to_coils_yaml,
    };

    let mb_regs = yaml_coils.load_from_yaml().unwrap();
    let vec_coils = mb_regs.coils;
    vec_coils
}

pub fn split_on_vecs() {
    // Поулчаем вектор настроек
    let vec_coils = read_yaml_coils();

    let mut coils: Coils;
    for vec in vec_coils {
        coils = vec;
    }
    println!("vector: {:#?}", coils);
}
