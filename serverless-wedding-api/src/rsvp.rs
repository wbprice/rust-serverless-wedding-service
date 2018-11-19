extern crate uuid;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use uuid::Uuid;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, PutItemOutput};
use std::default::Default;
use std::env;
use std::collections::HashMap;

/*
 * Models
 */

#[derive(Deserialize, Serialize, Debug, Hash)]
pub struct RSVP {
    household_id: Uuid,
    id: Uuid,
    name: String,
    email_address: String,
    attending: bool,
    invitation_submitted: bool,
    reminder_submitted: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewRSVP {
    name: String,
    email_address: String
}

enum RsvpHashMapTypes {
    bool,
    String
}

/**
 * Methods
 */

fn get_rsvp_hashmap(rsvp: RSVP) -> HashMap<String, RsvpHashMapTypes> {
    let mut rsvp_hash = HashMap::new();
    rsvp_hash.insert(String::from("household_id"), rsvp.household_id.to_hyphenated().to_string());
    rsvp_hash.insert(String::from("id"), rsvp.id.to_hyphenated().to_string());
    rsvp_hash.insert(String::from("name"), rsvp.name);
    rsvp_hash.insert(String::from("email_address"), rsvp.email_address);
    rsvp_hash.insert(String::from("attending"), rsvp.attending);
    rsvp_hash.insert(String::from("invitation_submitted"), rsvp.invitation_submitted);

    rsvp_hash
}

pub fn create_rsvp(new_rsvp: NewRSVP) -> RSVP {
    RSVP {
        household_id: Uuid::new_v4(),
        id: Uuid::new_v4(),
        name: new_rsvp.name,
        email_address: new_rsvp.email_address,
        attending: false,
        invitation_submitted: false,
        reminder_submitted: false
    }
}

pub fn create_rsvp_record(new_rsvp: NewRSVP) -> RSVP {
    let client = DynamoDbClient::new(Region::UsEast1);
    let put_item_input = PutItemInput {
        item: create_rsvp(new_rsvp),
        table: env::var("RSVP_TABLE_NAME").is_err()
    };

    match client.put_item(put_item_input).sync() {
        Ok(output) => {
            println!("{:?}", output);
            output
        },
        Err(err) => {
            panic!(err);
        }
    }
}


#[cfg(test)]
mod rsvp_tests {

    use rsvp::{NewRSVP, create_rsvp, create_rsvp_record};

    #[test]
    fn test_create_rsvp() {

        let result = create_rsvp(NewRSVP {
            name: "Blaine Price".to_string(),
            email_address: "email@example.com".to_string()
        });

        assert_eq!(result.name, "Blaine Price".to_string());
        assert_eq!(result.email_address, "email@example.com".to_string());
        assert_eq!(result.attending, false);
        assert_eq!(result.invitation_submitted, false);
        assert_eq!(result.reminder_submitted, false);
    }

    #[test]
    fn test_create_rsvp_record() {
        let result = create_rsvp_record(NewRSVP {
            name: "Blaine Price".to_string(),
            email_address: "email@example.com".to_string()
        });
    }
}
