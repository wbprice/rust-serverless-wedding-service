use serde_derive::{Serialize, Deserialize};
use std::vec::{Vec};
use std::collections::{HashMap};
use std::env;
use uuid::Uuid;
use log::{debug, info, error};
use std::error::Error;

use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDb,
    AttributeValue,
    QueryInput,
    QueryError,
    PutRequest,
    DynamoDbClient,
    WriteRequest,
    BatchWriteItemInput,
    BatchWriteItemError,
    UpdateItemInput,
    UpdateItemError
};
use serde_dynamodb;

mod person;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RSVP {
    household_id: String,
    id: String,
    name: String,
    email_address: String,
    attending: bool,
    invitation_submitted: bool,
    reminder_submitted: bool
}

impl RSVP {
    pub fn new(person : Person, household_id: String) -> RSVP {
        RSVP {
            household_id,
            id: Uuid::new_v4().to_string(),
            name: person.name,
            email_address: person.email_address,
            attending: false.into(),
            invitation_submitted: false.into(),
            reminder_submitted: false.into()
        }
    }

    pub fn patch(uuid: Uuid, payload: HashMap<String, bool>) -> Result<RSVP, UpdateItemError> {
        let client = DynamoDbClient::new(Region::UsEast1);

        let rsvp = RSVP::get(uuid).unwrap();

        debug!("Preparing to update RSVP: {:?}", rsvp);

        // Get primary key for update operation
        let mut key = HashMap::new();
        key.insert(String::from("household_id"), AttributeValue {
            s: Some(String::from(rsvp.clone().household_id)),
            ..Default::default()
        });
        key.insert(String::from("name"), AttributeValue {
            s: Some(String::from(rsvp.clone().name)),
            ..Default::default()
        });

        // Create the update expression from the payload
        // TODO: Is there an idiomatic way to do this better with Rust?
        let mut update_expression = String::from("SET ");
        let payload_iter = payload.iter();
        let iter_length = payload_iter.clone().count();
        let mut payload_iter_count = 0;
        for (key, _) in payload_iter {
            let mut append = format!("{k} = :{k}", k = key);
            payload_iter_count = payload_iter_count + 1;
            if payload_iter_count != iter_length {
                append.push_str(",");
            }
            update_expression.push_str(&append);
        }

        // Create the expression attributes value hashmap from the payload
        let mut expression_attribute_values = HashMap::new();
        for (key, value) in payload {
            expression_attribute_values.insert(String::from(format!(":{}", key.to_string())), AttributeValue {
                bool: Some(value),
                ..Default::default()
            });
        }

        // Gather the above into an instance of UpdateItemInput
        let update_item_input = UpdateItemInput {
            key,
            update_expression: Some(String::from(update_expression)),
            expression_attribute_values: Some(expression_attribute_values),
            table_name: env::var("RSVP_TABLE_NAME").unwrap(),
            ..Default::default()
        };

        info!("Running client.update_item");

        // Perform the request!
        match client.update_item(update_item_input).sync() {
            Ok(_response) => {
                // If the PUT was successful, fetch the updated record and return it
                info!("Success!");
                Ok(RSVP::get(uuid).unwrap())
            },
            Err(error) => {
                error!("Error! {:?}", error);
                Err(error)
            }
        }
    }

    pub fn get(uuid: Uuid) -> Result<RSVP, QueryError> {
        let client = DynamoDbClient::new(Region::UsEast1);
        
        let mut query = HashMap::new();
        query.insert(String::from(":id"), AttributeValue {
            s: Some(uuid.to_string()),
            ..Default::default()
        });

        info!("Preparing to get a record of UUID: {:?}", uuid);

        let query_input = QueryInput {
            index_name: Some(env::var("RSVP_TABLE_ID_INDEX_NAME").unwrap()),
            table_name: env::var("RSVP_TABLE_NAME").unwrap(),
            key_condition_expression: Some("id = :id".to_string()),
            expression_attribute_values: Some(query),            
            ..Default::default()
        };

        info!("Query Input is {:?}", query_input);

        let rsvps : Vec<RSVP> = match client.query(query_input).sync() {
            Ok(response) => {
                match response.items {
                    Some(items) => {
                        info!("Some results were found! {:?}", items);
                        let rsvps = items.into_iter()
                            .map(|item| serde_dynamodb::from_hashmap(item).unwrap())
                            .collect();
                        rsvps
                    },
                    None => {
                        error!("No results!");
                        vec![]
                    }
                }
            },
            Err(err) => {
                error!("There was an error performing the query! {}", err);
                vec![]
            }
        };

        if rsvps.len() == 0 {
            Err(QueryError::ResourceNotFound(String::from("No matches")))
        } else {
            Ok(rsvps[0].clone())
        }
    }
}


#[cfg(test)]
mod rsvp_tests {

    use super::*;

    #[test]
    fn test_rsvp_new() {
        let household_id = Uuid::new_v4().to_string();
        let result = RSVP::new(
            Person {
                name: "Blaine Price".to_string(),
                email_address: "email@example.com".to_string()
            },
            household_id.clone()
        );

        assert_eq!(result.name, "Blaine Price".to_string());
        assert_eq!(result.email_address, "email@example.com".to_string());
        assert_eq!(result.household_id, household_id);
        assert_eq!(result.attending, false);
        assert_eq!(result.invitation_submitted, false);
        assert_eq!(result.reminder_submitted, false);
    }
    
    #[test]
    fn test_household_new() {
        let people : Vec<Person> = vec!(
            Person {
                email_address: "1example@email.com".to_string(),
                name: "person 1".to_string()
            },
            Person {
                email_address: "2example@email.com".to_string(),
                name: "person 2".to_string()
            }
        );

        let rsvps = Household::new(people);
        assert_eq!(rsvps[0].household_id, rsvps[1].household_id);
    }

    #[test]
    fn test_household_create_records() {
        let people : Vec<Person> = vec!(
            Person {
                email_address: "1example@email.com".to_string(),
                name: "person 1".to_string()
            },
            Person {
                email_address: "2example@email.com".to_string(),
                name: "person 2".to_string()
            }
        );

        let rsvps = RSVP::batch_create_records(people).unwrap();
        assert_eq!(rsvps[0].household_id, rsvps[1].household_id);
    }

    #[test]
    fn test_household_get() {
        let uuid = Uuid::parse_str("3eb28445-7698-4a00-b071-49da8eaac944").unwrap();
        let rsvps = RSVP::list_by_household_id(uuid).unwrap();
        assert_eq!(rsvps.len(), 2);
    }

    #[test]
    fn test_rsvp_patch() {
        let uuid = Uuid::parse_str("955e9465-d9cc-43cc-96ac-0fe00fc75d0e").unwrap();
        let mut payload : HashMap<String, bool> = HashMap::new();
        payload.insert(String::from("attending"), true);
        payload.insert(String::from("invitation_submitted"), true);
        payload.insert(String::from("reminder_submitted"), true);

        match RSVP::patch(uuid, payload.clone()) {
            Ok(rsvp) => {
                assert_eq!(&rsvp.attending, payload.get("attending").unwrap());
                assert_eq!(&rsvp.invitation_submitted, payload.get("invitation_submitted").unwrap());
                assert_eq!(&rsvp.reminder_submitted, payload.get("reminder_submitted").unwrap());
            },
            Err(err) => {
                println!("The update error is {:?}", err);
            }
        }
    }

    #[test]
    fn test_rsvp_get() {
        let uuid = Uuid::parse_str("955e9465-d9cc-43cc-96ac-0fe00fc75d0e").unwrap();
        
        match RSVP::get(uuid) {
            Ok(rsvp) => {
                println!("the results are {:?}", rsvp);
            },
            Err(err) => {
                println!("Get test");
                println!("The error is {:?}", err);
            }
        }
    }
}
