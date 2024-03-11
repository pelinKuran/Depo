use create::entity::{prelude::*,storage};
use actix_web::web::Json;
use log::debug;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveValue::NotSet};
use sea_orm::{entity::*, query::*, DeriveEntityModel, DeleteResul};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageRequest {
    pub title: String,
    pub completed: bool,
}

#[derive(Debug,Clone)]
pub struct StorageRepository {
    pub db: DatabaseConnection,
}
impl StorageRepository {
    pub async fn get_storage(&self) -> Vect<storage::Model>{
        Storage::find()
            .all(&self.db)
            .await
            .expect("Failed to get storage")
    }    
    pub async fn get_storage_item_by_id(&self, id: i32) -> Option<storage::Model>   {
        Storage::find_by_id(id)
            .one(&self.db)
            .await
            .expect("Failed to get storage item")
    }
    pub async fn create_storage_item(&self, new_item: Json<StorageRequest>) -> Option<storage::Model>{
        let 
    }
}