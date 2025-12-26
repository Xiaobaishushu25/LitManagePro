use crate::entities::prelude::{DocAndTags, Documents, TagGroups, Tags};
use log::{error, info};
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait, Schema};

async fn create_table<E>(db_connection: &sea_orm::DatabaseConnection, entity: E)
where
    E: EntityTrait,
{
    let backend = db_connection.get_database_backend();
    let schema = Schema::new(backend);
    let execution = db_connection.execute(backend.build(&schema.create_table_from_entity(entity)));
    match execution.await {
        Ok(_) => info!("Created {} table.", entity.table_name()),
        Err(e) => {
            error!("create data table error: {}", e);
            panic!("create data table error: {}", e);
        }
    }
}
pub async fn create_all_need_table(db: &DatabaseConnection) {
    create_table(db, Tags).await;
    create_table(db, TagGroups).await;
    create_table(db, Documents).await;
    create_table(db, DocAndTags).await;
    // let _ = create_table(db, StockGroups).await;
    // let _ = create_table(db, GroupStockRs).await;
    // let _ = create_table(db, TransactionRecords).await;
    // let _ = create_table(db, Positions).await;
    // StockGroupCurd::insert_init(db).await.unwrap();
}
#[cfg(test)]
mod test {
    use crate::entities::prelude::{Documents};
    use crate::entities::table::create_table;
    use crate::entities::{DB, init_db_coon};
    #[tokio::test]
    async fn test_create_table() {
        init_db_coon().await;
        let db = DB.get().unwrap();
        // create_table(db, DocAndTags).await;
        create_table(db, Documents).await;
    }
}
