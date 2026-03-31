use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)] /// Represents a rule in the system

pub struct Rules{
    /// List of required column names in the csv file
    pub required_columns: Vec<String>, 

    /// List of required values in the csv file
    pub required_values: Vec<String>,

    /// Fields must pare to numeric values (i64)
    pub numeric_fields: Vec<String> 
}