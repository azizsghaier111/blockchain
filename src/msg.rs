use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    SetUserVector { value: i32 },
    SetAdminVector { admin_vector: Vec<i32> },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCount {},
    GetUserVector {},
    GetAdminVector {},
    GetDotProduct {},
}

#[derive(Serialize, Deserialize, Clone, Eq, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Eq, Debug, PartialEq, JsonSchema)]
pub struct VectorResponse {
    pub vector: Vec<i32>,
}
pub struct AdminVectorResponse {
    pub vector: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Eq, Debug, PartialEq, JsonSchema)]
pub struct DotProductResponse {
    pub dot_product: i32,
}