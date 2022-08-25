use std::str::FromStr;

use email_address::EmailAddress;

fn main() {
    create_person_and_log(
        "Martin", 
        "Othamar", 
        "martin.othamar@forse.no"
    );

    create_person_and_log(
        "Martin", 
        "Othamar", 
        "martin.othamarforse.no"
    );
}

pub fn create_person_and_log(first_name: &str, last_name: &str, email: &str) {
    let person_result = Person::new(
        first_name, 
        last_name, 
        email
    );

    match person_result {
        Ok(person) => println!("Created person: {:?}", person),
        Err(err) => {
            if let CreatePersonError::InvalidEmail(_) = err {
                // Something else
            }
            println!("Failed creating person: {:?}", err);
        },
    }
}

#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
    email: EmailAddress,
}

#[derive(Debug, PartialEq)]
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