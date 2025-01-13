use std::env;

pub fn file_path() -> String {
    env::var("AWS_CONFIG_FILE").unwrap_or_else(|_| {
        let home = dirs::home_dir().unwrap();
        let mut path = home.clone();
        path.push(".aws");
        path.push("config");
        path.to_str().unwrap().to_string()
    })
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_file_path() {
        let file = file_path();
        assert_eq!(file, "/home/newgyu/.aws/config");
    }
}
