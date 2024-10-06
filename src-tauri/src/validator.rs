use regex::Regex;

pub fn validate_email(email: &str) -> bool {
    let re = Regex::new(r"^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$").unwrap();

    re.is_match(email)
}

pub fn validate_password(password: &str) -> bool {
    let re = Regex::new(
        r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[!@#$%^&*()_+{}\[\]:;<>,.?~\\\/\-]).{8,}$",
    )
    .unwrap();

    re.is_match(password)
}
