use serde::{Deserialize, Serialize};

/* FileInfo:
  This is to be used to easily structure the wordsets-data.json file
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WordsetInfo {
  pub name: String,
  pub path: String,
}