

/*

// Focussing on two types of database use
// local - sqlite database
// remote - externally hosted sql

*/

use crate::config;

fn get_db_path() -> String {
    let config_data = config::file::get_application_config();
    let path = config_data.database_path.to_owned();
    
    path
}


pub mod local {
    use std::fs;

    #[derive(serde::Serialize)]
    pub enum ConnectionStatus {
        Success,
        Fail,
    }

    pub fn test_connection() -> ConnectionStatus{
    
        let connection_path = super::get_db_path();
    
        match sqlite::open(connection_path) {
            Ok(_) => ConnectionStatus::Success,
            Err(_) => ConnectionStatus::Fail,
        }
    
    
    }

    /// Check that the SQLite file exists
    pub fn sqlite_db_exists() -> bool {
        let path_name = super::get_db_path();
        let exists = fs::metadata(path_name).is_ok();

        exists
    }

    
}