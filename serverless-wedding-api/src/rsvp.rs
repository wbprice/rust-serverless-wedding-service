use serde_derive::{Serialize, Deserialize};
use std::vec::{Vec};
use std::collections::{HashMap};
use std::env;
use uuid::Uuid;
use log::{info, error};
use std::error::Error;

use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDb,
    AttributeValue,
    GetItemInput,
    GetItemError,
    QueryInput,
    QueryError,
    PutRequest,
    PutItemError,
    DynamoDbClient,
    WriteRequest,
    BatchWriteItemInput,
    BatchWriteItemError,
    UpdateItemInput,
    UpdateItemError
};
use serde_dynamodb;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    email_address: String,
    name: String
}

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

    pub fn batch_new(people: Vec<Person>) -> Vec<RSVP> {
        let uuid = Uuid::new_v4().to_string();
        let mut rsvps : Vec<RSVP> = vec!();
        
        for person in people {
            rsvps.push(RSVP::new(person, uuid.clone()).clone());
        }

        rsvps
    }

    // pub fn patch_rsvp(uuid: Uuid) -> Result<RSVP, UpdateItemError> {
    //     let client = DynamoDbClient::new(Region::UsEast1);

    //     // Get primary key for update operation
    //     let key = HashMap::new();
    //     key.insert(String::from("household_id"), AttributeValue {
    //         s: Some(uuid.to_string()),
    //         ..Default::default()
    //     });
    //     key.insert(String::from("name"), AttributeValue {
    //         s: Some(uuid.to_string()),
    //         ..Default::default()
    //     });

    //     // Create update expression and values
    //     let update_expression = "SET attending = :attending";
    //     let update_attribute_values = HashMap::new();
    //     update_attribute_values.insert(String::from(":attending"), AttributeValue {
    //         BOOL: rsvp.attending,
    //         ..Default::default()
    //     });

    //     // Gather the above into an instance of UpdateItemInput
    //     let update_item_input = UpdateItemInput {
    //         key,
    //         update_expression,
    //         update_attribute_values
    //     }

    //     // Perform the request!
    //     match client.update_item(update_item_input).sync() {
    //         Ok(response) => {
    //             println!("{:?}", response);
    //             rsvp
    //         },
    //         Err(error) => error
    //     }
    // }

    pub fn get(uuid: Uuid) -> Result<RSVP, GetItemError> {
        let client = DynamoDbClient::new(Region::UsEast1);
        
        let mut key = HashMap::new();
        key.insert(String::from("id"), AttributeValue {
            s: Some(uuid.to_string()),
            ..Default::default()
        });

        let get_item_input = GetItemInput {
            key,
            ..Default::default()
        };

        match client.get_item(get_item_input).sync() {
            Ok(get_item_output) => {
                match get_item_output.item {
                    Some(item) => { 
                        let rsvp : RSVP = serde_dynamodb::from_hashmap(item).unwrap();
                        Ok(rsvp)
                    }
                    None => {
                        panic!("no results");
                    }
                }
            },
            Err(err) => Err(err)
        }
    }

    pub fn list_by_household_id(uuid: Uuid) -> Result<Vec<RSVP>, Box<Error>> {
        let client = DynamoDbClient::new(Region::UsEast1);

        let mut query = HashMap::new();
        query.insert(String::from(":household_id"), AttributeValue {
            s: Some(uuid.to_string()),
            ..Default::default()
        });

        let query_input = QueryInput {
            table_name: env::var("RSVP_TABLE_NAME").unwrap(),
            key_condition_expression: Some("household_id = :household_id".to_string()),
            expression_attribute_values: Some(query),
            ..QueryInput::default()
        };

        match client.query(query_input).sync() {
            Ok(response) => {
                match response.items {
                    Some(items) => {
                        let rsvps = items.into_iter()
                            .map(|item| serde_dynamodb::from_hashmap(item).unwrap())
                            .collect();
                        Ok(rsvps)
                    },
                    None => {
                        error!("No results!");
                        Ok(vec![])
                    }
                }
            },
            Err(error) => {
                error!("There was an error performing the query {}", error);
                Ok(vec![])
            }
        }
    }
    
    pub fn batch_create_records(people: Vec<Person>) -> Result<Vec<RSVP>, BatchWriteItemError> {
        let rsvps = RSVP::batch_new(people); 
        let client = DynamoDbClient::new(Region::UsEast1);

        let mut put_requests : Vec<WriteRequest> = vec!();
        for rsvp in &rsvps {
            put_requests.push(
                WriteRequest {
                    put_request: Some(PutRequest {
                        item: serde_dynamodb::to_hashmap(&rsvp).unwrap()
                    }),
                    ..WriteRequest::default()
                }
            )
        }

        let mut request_items : HashMap<String, Vec<WriteRequest>> = HashMap::new();
        request_items.insert(env::var("RSVP_TABLE_NAME").unwrap(), put_requests);

        let batch_write_request_input = BatchWriteItemInput {
            request_items: request_items,
            ..BatchWriteItemInput::default()
        };

        match client.batch_write_item(batch_write_request_input).sync() {
            Ok(_result) => {
                Ok(rsvps)
            },
            Err(error) => {
                Err(error)
            }
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
    fn test_rsvp_batch_new() {
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

        let rsvps = RSVP::batch_new(people);
        assert_eq!(rsvps[0].household_id, rsvps[1].household_id);
    }

    #[test]
    fn test_rsvp_batch_create_records() {
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
    fn test_rsvp_list_by_househould_id() {
        let uuid = Uuid::parse_str("3eb28445-7698-4a00-b071-49da8eaac944").unwrap();
        let rsvps = RSVP::list_by_household_id(uuid).unwrap();
        assert_eq!(rsvps.len(), 2);
    }

    #[test]
    // fn test_patch_rsvp() {
    //     let uuid = Uuid::parse_str("3eb28445-7698-4a00-b071-49da8eaac944").unwrap();
    // }

    #[test]
    fn test_get() {
        let uuid = Uuid::parse_str("955e9465-d9cc-43cc-96ac-0fe00fc75d0e").unwrap();
        
        match RSVP::get(uuid) {
            Ok(rsvp) => {
                println!("{:?}", rsvp);
            },
            Err(err) => {
                println!("The error is {:?}", err);
            }
        }
    }
}