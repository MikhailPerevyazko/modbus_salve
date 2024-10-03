use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigArgs {
    // Путь к файлу конфигурации
    pub config_path: Option<PathBuf>,
}

impl ConfigArgs {
    // Метод возвращает путь к файлу конфигурации
    pub fn get_path_to_config(&self) -> PathBuf {
        match &self.config_path {
            None => PathBuf::from(
                "/home/Mikhail/projects/try_to_modbus/async-modbus-tcp-server/config.yaml",
            ),
            Some(path) => path.to_owned(),
        }
    }
    // Метод проверяет существует ли путь к конфигурационному файлу
    pub fn check_path_to_config(&self) {
        match &self.config_path {
            None => {
                println!(
                    "Путь к конфигурационному файлу не задан. Используется путь по умолчанию.\n"
                )
            }
            Some(path) => println!("Пусть к файлу с конфигом: {:?}\n", path),
        };
    }
}
