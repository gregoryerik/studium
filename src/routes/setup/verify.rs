/*
//
// Verify the data sent from the setup form
// Verify that the data is properly formatted, isn't malicious and the db exists
//
 */

use regex::Regex;

use crate::{database::{self, local::ConnectionStatus}, config};

pub enum Codes {
    ALLOW,
    REJECT,
    MODIFY(String),
}

pub fn verify_name(name: String) -> Codes {
    let regex_query = Regex::new(r"[^a-zA-Z]").unwrap();
    let replaced = regex_query.replace_all(&name, "").to_string();

    // If the strings havent changed, then simply 
    if name == replaced {
        // Can simply allow the string to be used in its original form as changing the string doesn't matter
        Codes::ALLOW
    } else {
        // Request that the string be modified to the new value as it contained disallowed characters
        Codes::MODIFY(replaced)
    }
}

pub fn verify_email(email: String) -> Codes {
    let re = Regex::new(r#"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$"#)
        .unwrap();

    if re.is_match(&email) {
        // Email is valid, remove any potentially harmful characters
        let sanitized_email = email
            .trim() // Remove leading and trailing whitespace
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("&", "&amp;")
            .replace("\"", "&quot;")
            .replace("'", "&#x27;")
            .replace("/", "&#x2F;");

        Codes::MODIFY(sanitized_email)
    } else {
        Codes::REJECT
    }

}

pub fn verify_sql_path(sql_path: String) -> Codes {

    // if the db path presented is empty, then create on in app. therefore create and return accept
    // if not empty. Check if the file exists. If not. Reject
    let new_path: String;

    if sql_path.is_empty() {
        // then this can be created
        new_path = "database.db".to_string();
        database::set_db_path(new_path.clone());
    } else {

        // find the file

        let md = std::fs::metadata(sql_path.clone()).unwrap();

        if !md.is_file() { 
            return Codes::REJECT;
        }
        new_path = sql_path.clone();
        database::set_db_path(sql_path.clone());
    }

    // attempt connection
        
    return match database::local::test_connection() {
        ConnectionStatus::Success => {
            if sql_path == new_path {
                Codes::ALLOW
            } else {
                Codes::MODIFY(new_path)
            }
        },
        ConnectionStatus::Fail => Codes::REJECT,
    };
    
}

