use crate::permissions::ALL_PERMISSIONS;
use taskrs_db::models::permission::dtos::PermissionCreate;
use taskrs_db::sea_orm::{ColumnTrait, Condition, ConnectionTrait, DbConn, DbErr};

pub async fn seed_permissions(db: &DbConn) {
    info!("Seeding permissions from bundled permissions");

    let json_permissions: Vec<PermissionCreate> = serde_json::from_str(ALL_PERMISSIONS)
        .unwrap_or_else(|err| {
            error!("Error while deserializing embedded permissions.");
            error!("{}", err);
            panic!();
        });

    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            debug!("Geting current permissions");
            let db_permissions = taskrs_db::actions::permissions::get_all(None, None, txn).await?;

            // Compare to get new permissions
            debug!("Finding new permissions");
            let new_permissions: Vec<PermissionCreate> = json_permissions
                .iter()
                .filter(|p| {
                    !db_permissions
                        .iter()
                        .any(|dbp| dbp.group == p.group && dbp.name == p.name)
                })
                .cloned()
                .collect();

            // Compare to get old permissions
            debug!("Finding old permissions");
            let old_permission_ids: Vec<i32> = db_permissions
                .iter()
                .filter(|p| {
                    !json_permissions
                        .iter()
                        .any(|jp| jp.group == p.group && jp.name == p.name)
                })
                .map(|p| p.id)
                .collect();

            // Insert new permissions
            debug!("Inserting new permissions");
            for new_permission in new_permissions {
                taskrs_db::actions::permissions::create(new_permission, txn).await?;
            }

            // Remove old permissions
            debug!("Removing old permissions");
            taskrs_db::actions::permissions::delete(
                None,
                Some(
                    Condition::all()
                        .add(taskrs_db::models::permission::Column::Id.is_in(old_permission_ids)),
                ),
                txn,
            )
            .await?;

            Ok(())
        })
    })
    .await
    .unwrap_or_else(|err| {
        error!("Database error while seeding permissions.");
        error!("{}", err);
        panic!();
    });
}
