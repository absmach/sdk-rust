// src/main.rs

mod apis {  // Define the api module
    pub mod examples; 
    pub mod configuration; // Include the examples.rs file from the api directory
}

#[tokio::main]
async fn main() {
    // Call the function from examples module
    api::examples::run_example();  
}
