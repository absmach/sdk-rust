mod models;
use models::UserReqObj;// Replace with the actual paths to your models
use models::user_req_obj_credentials::UserReqObjCredentials;

mod apis;
use apis::configuration::Configuration;
 use crate::apis::users_api::create_user;
use serde_json::json;

// Replace with the actual path to your configuration



#[tokio::main]
async fn main() {
    // Construct the configuration with the actual API base URL and authorization token
    let mut config = Configuration::new();
    config.base_path = "http://localhost:9002".to_string();

    // Create the credentials, wrapping each field inside Some
    let credentials = Box::new(UserReqObjCredentials {
        username: Some("admin".to_string()), // Wrap in Some()
        secret: Some("password".to_string()), // Wrap in Some()
    });
    let metadata = Some(json!({
        "domain": "example.com",}));

    // Create the user request object
    let user_req_obj = UserReqObj {
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("johnde@example.com".to_string()),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        credentials: credentials,  // Boxed credentials
        metadata: metadata, 
        profile_picture: Some("https://example.com/profile.jpg".to_string()),
        status: Some("enabled".to_string()),

};


    // Call the `create_user` function
    match create_user(&config, user_req_obj).await {
        Ok(user) => {
            // Success: The user has been created, and `user` contains the response data
            println!("User created successfully: {:?}", user);
        }
        Err(err) => {
            // Handle the error (e.g., logging, retrying, etc.)
            eprintln!("Error creating user: {:?}", err);
        }
    }
}