use std::env;

pub fn get_aws_config_file() -> String {
    env::var("AWS_CONFIG_FILE").unwrap_or_else(|_| {
        let home = dirs::home_dir().unwrap();
        let mut path = home.clone();
        path.push(".aws");
        path.push("config");
        path.to_str().unwrap().to_string()
    })
}

pub fn get_aws_credentials_file() -> String {
    env::var("AWS_SHARED_CREDENTIALS_FILE").unwrap_or_else(|_| {
        let home = dirs::home_dir().unwrap();
        let mut path = home.clone();
        path.push(".aws");
        path.push("credentials");
        path.to_str().unwrap().to_string()
    })
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_get_aws_config_file() {
        let file = get_aws_config_file();
        assert_eq!(file, "/home/newgyu/.aws/config");
    }

    #[test]
    fn test_get_aws_credentials_file() {
        let file = get_aws_credentials_file();
        assert_eq!(file, "/home/newgyu/.aws/credentials");
    }
}
