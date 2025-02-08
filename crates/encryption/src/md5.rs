use md5;

pub struct Md5;

impl Md5 {
    pub fn hash(input: &String, salt: Option<&String>) -> String {
        if let Some(salt) = salt {
            let input = format!("{}{}", input, salt);
            format!("{:x}", md5::compute(input))
        } else {
            format!("{:x}", md5::compute(input))
        }
    }
}
