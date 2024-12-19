mod models;
use models::user_req_obj_credentials::UserReqObjCredentials;
use models::UserReqObj; // Replace with the actual paths to your models

mod apis;
use crate::apis::users_api::*;
use apis::configuration::Configuration;
use models::issue_token::IssueToken;
use serde_json::json;

// Replace with the actual path to your configuration

#[tokio::main]
async fn main() {
    // Construct the configuration with the actual API base URL and authorization token
    let mut config = Configuration::new();
    config.base_path = "http://localhost:9002".to_string();
    config.bearer_access_token = Some("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MzQ0MjU2MTUsImlhdCI6M\
    TczNDQyMjAxNSwiaXNzIjoic3VwZXJtcS5hdXRoIiwidHlwZSI6MCwidXNlciI6IjRlMjZiMzM4LTg0OGEtNGM1Yi04OWFlLWUyY\
    WUxODhmNDhiMyJ9.cLYQUPrab8URh_FdtSyE15ZrnwJCUBEtwfAwn4CDjIGuVqVRVwoBr1xV8FfuqDxyfpPr-86zQJUP1b4Vqv8T9g".to_string());

    // Create the credentials, wrapping each field inside Some
    let credentials = Box::new(UserReqObjCredentials {
        username: Some("adminDorc".to_string()), // Wrap in Some()
        secret: Some("password".to_string()),    // Wrap in Some()
    });

    let metadata = Some(json!({
        "domain": "example.com",
    }));
    let cloned_credentials = credentials.clone();

    // Create the user request object
    let user_req_obj = UserReqObj {
        first_name: Some("Njeri".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("Njeri@example.com".to_string()),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        credentials: credentials, // Boxed credentials
        metadata: metadata,
        profile_picture: Some("https://example.com/profile.jpg".to_string()),
        status: Some("enabled".to_string()),
    };
    let token_request = IssueToken::new(
        cloned_credentials.username.as_ref().unwrap_or(&"".to_string()).clone(),
        cloned_credentials.secret.as_ref().unwrap_or(&"".to_string()).clone(),
    );

    // // Call the `create_user` function
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

    // //Obtain token
    // match issue_token(&config, token_request).await {
    //     Ok(user) => {
    //         // Success: The user has been created, and `user` contains the response data
    //         println!("Token generated successfully: {:?}", user);
    //     }
    //     Err(err) => {
    //         // Handle the error (e.g., logging, retrying, etc.)
    //         eprintln!("Error generating token: {:?}", err);
    //     }
    // }

    //Get user
    // let user_response: User =  match get_profile(&config).await {
    //     Ok(user) => {
    //         // Success: The user has been created, and `user` contains the response data
    //         return user;
    //     }
    //     Err(err) => {
    //         // Handle the error (e.g., logging, retrying, etc.)
    //         eprintln!("Error obtaining: {:?}", err);
    //     }
    // }
  

    // if let Some(id) = user_response.id {
    //     println!("User ID: {}", id);
    // } else {
    //     println!("User ID is None");
   // }

    // match get_user(&config,).await {
    //     Ok(user) => {
    //         // Success: The user has been created, and `user` contains the response data
    //         println!("Obtained user successfully: {:?}", user);
    //     }
    //     Err(err) => {
    //         // Handle the error (e.g., logging, retrying, etc.)
    //         eprintln!("Error obtaining: {:?}", err);
    //     }
    // }

}

