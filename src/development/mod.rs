
/*
 Tools for use only in development. Not used for any critical funcationality that has any impact on the application

*/

pub mod machine {

    use fast_qr::{convert::ConvertError, QRBuilder};
    use local_ip_address::local_ip;

    fn get_ip_addr() -> String {
        let local_ip = local_ip().unwrap().to_string();

        local_ip
    }

    fn gen_qr_code(data: String) -> Result<(), ConvertError> {

        println!("[url] {}", data);

        let qr_code = QRBuilder::new(data)
            .build()
            .unwrap();

        let str = qr_code.to_str();
        println!("{}", str);

        Ok(())
    }

    pub fn get_server_url_qr() {
        let ip_addr = get_ip_addr();
        let url = format!("http://{}:8080/", ip_addr);
        match gen_qr_code(url) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        };
    }

    pub fn get_server_url() -> String {
        let ip_addr = get_ip_addr();
        let url = format!("http://{}:8080/", ip_addr);

        url

    }
}


