

/*

// Focussing on two types of database use
// local - sqlite database
// remote - externally hosted sql

*/


pub mod local {
    use crate::config;
    
    #[derive(serde::Serialize)]
    pub enum ConnectionStatus {
        Success,
        Fail,
    }
    
    fn get_connection_path() -> String {
        let config_data = config::file::get_application_config();
        let path = config_data.database_path.to_owned();
        
        path
    }
    
    pub fn test_connection() -> ConnectionStatus{
    
        let connection_path = get_connection_path();
    
        match sqlite::open(connection_path) {
            Ok(_) => ConnectionStatus::Success,
            Err(_) => ConnectionStatus::Fail,
        }
    
    
    }
}

pub mod remote {

}