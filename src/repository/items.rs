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
        let item = storage::ActiveModel
        {
            id: Notset,
            title: ActiveModel::Set(new_item.title.to_owned()),
            text: ActiveValue::Set(new_item.text.to_owned()),
        };
        let item: storage::Model = item.insert(&self.db).await.unwrap();
        debug!("Created storage: item{}", item.title);
        return item.into();
    }
    pub async fn update_storage_item(&self, id:i32, new_item: Json<TodoRequest>) -> Option<storage::Model>
    {
        let item = Storage::find_by_id(id)
            .one(&self.db)
            .await
            .expect("Failed to get item");
        let mut item: storage::Model = item.update(&self.db).await.unwrap();
        item.title = ActiveModel::Set(new_item.title.to_owned());
        item.text = ActiveValue::Set(new_item.text.to_owned());
        let item: item::Model = storage.update(&self.db).await.unwrap();
        debug!("Updated item: item{}", item.title);
        return item.into();
    }
    pub async fn delete_item_by_id(&self, id: i32) -> DeleteResult {
        let res: DeleteResult = Storage::delete_item_by_id(id).exec(&self.db).await.unwrap();
        return res.into();

    }


}