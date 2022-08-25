use std::str::FromStr;

use email_address::EmailAddress;

fn main() {
    let mut person_result = Person::new(
        "Martin", 
        "Othamar", 
        "martin.othamar@forse.no"
    );
    println!("{:?}", person_result);

    person_result = Person::new(
        "Martin", 
        "Othamar", 
        "martin.othamarforse.no"
    );
    println!("{:?}", person_result);
}

#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
    email: EmailAddress,
}

#[derive(Debug)]
pub enum CreatePersonError {
    NameTooShort,
    NameTooLong,
    InvalidEmail(email_address::Error)
}

type CreatePersonResult = Result<Person, CreatePersonError>;

impl Person {
    pub fn new(first_name: &str, last_name: &str, email: &str) -> CreatePersonResult {
        if first_name.is_empty() || last_name.is_empty() {
            return Err(CreatePersonError::NameTooShort);
        }
        if first_name.len() + last_name.len() > 128 {
            return Err(CreatePersonError::NameTooLong);
        }

        let email = EmailAddress::from_str(email)
            .map_err(|e| CreatePersonError::InvalidEmail(e))?;

        Ok(Self { 
            first_name: String::from(first_name), 
            last_name: String::from(last_name),
            email: email,
        })
    }

    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }
    pub fn get_last_name(&self) -> &str {
        &self.first_name
    }
    pub fn get_email(&self) -> &EmailAddress {
        &self.email
    }
}