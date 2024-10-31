use serde::Deserialize;
use std::{error::Error, path::PathBuf};

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

#[derive(Debug, Clone, Deserialize)]
pub struct ModbusRegistersMap {
    pub channels: Vec<Coils>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Coils {
    pub parameters_name: String,
    pub unit_id: u8,
    pub start_address: u16,
    pub type_storage: String,
    pub parameters_type: String,
}

impl ModbusRegistersMap {
    // Получение вектора Coils по полю parametres name
    pub fn get_coil_by_name(&self, param_name: String) -> Option<&Coils> {
        let coils = self
            .channels
            .iter()
            .find(|coil| coil.parameters_name == param_name);

        coils
    }
}

pub fn call_to_reg_map(find_by_param_name: String) -> Coils {
    // Чтение карты регистров по заданному пути
    let path_to_coils_yaml: PathBuf =
        PathBuf::from("/home/Mikhail/projects/try_to_modbus/modbus_tcp/modbus_registers.yaml");

    let yaml_coils = YamlCoilsPath {
        file_path: path_to_coils_yaml,
    };

    // Запись карты регистров в структуру
    let mb_regs_map = yaml_coils.load_from_yaml().unwrap();

    // Находим по имени и записываем нужную нам карту
    let map = mb_regs_map
        .get_coil_by_name(find_by_param_name)
        .unwrap()
        .clone();

    map
}
