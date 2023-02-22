/*

    Load the configuration file and get out the data

*/

pub mod file {

    use serde_derive::Deserialize;
    use std::{fs, process::exit};
    use toml;

    const FILENAME: &str = "config.toml";

    // Server

    #[derive(Deserialize)]
    struct ServerContainer {
        server: Server,
    }

    impl ServerContainer {
        fn create_empty() -> ServerContainer {
            ServerContainer { server: Server  {
                ip: "".to_string(),
                port: 0,
            }}
        }
    }

    #[derive(Deserialize)]
    pub struct Server {
        pub ip: String,
        pub port: u16,
    }

    // Application

    #[derive(Deserialize)]
    struct ApplicationContainer {
        application: Application,
    }

    impl ApplicationContainer {
        fn create_empty() -> ApplicationContainer {
            ApplicationContainer {
                application: Application { mode: "".to_string(), database_path: "".to_string() }
            }
        }
    }

    #[derive(Deserialize)]
    pub struct Application {
        pub mode: String,
        pub database_path: String,
    }

    fn load_config_file() -> String {
        let config_data = match fs::read_to_string(FILENAME) {
            Ok(d) => d,
            Err(e) => {
                println!("Configuration file read error: {}", e);
                String::from("ERR")
            }
        };

        config_data
    }

    fn cd_is_ok(config_data: &String) -> bool {
        !(config_data == "ERR")
    }

    pub fn get_server_config() -> Server {
        let config_data = load_config_file();

        if config_data != "ERR".to_string() {
            let toml_data: ServerContainer = match toml::from_str(&config_data) {
                Ok(d) => d,
                Err(e) => {
                    println!("Error parsing config data: {}", e);
                    ServerContainer::create_empty()
                }
            };

            return toml_data.server;
        } else {
            exit(0);
        }

    }

    pub fn get_application_config() -> Application {
        let config_data = load_config_file();

        if cd_is_ok(&config_data) {
            let toml_data: ApplicationContainer = match toml::from_str(&config_data) {
                Ok(d) => d,
                Err(e) => {
                    println!("Error parsing config data: {}", e);
                    ApplicationContainer::create_empty()
                }
            };

            return toml_data.application
        }
        else {
            exit(0);
        }
    }

    pub fn write_application_config(app_config: Application) {
        // get entire toml file
        let toml_file = super::file::load_config_file();
        let mut parsed_toml: toml::Value = toml_file.parse().expect("Failed to parse TOML");

        //create the application table
        let mut app_table = toml::value::Table::new();
        app_table.insert("database_path".to_string(), toml::Value::from(app_config.database_path));
        app_table.insert("mode".to_string(), toml::Value::from(app_config.mode));

        // save these values to the table var
        parsed_toml.as_table_mut().unwrap().insert("application".to_string(), toml::Value::from(app_table));

        //write to file
        let toml_str = toml::to_string_pretty(&parsed_toml).unwrap();
        fs::write(FILENAME, toml_str).expect("Failed to write to file");
    }
}
