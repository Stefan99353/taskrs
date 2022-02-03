use crate::permissions::ALL_PERMISSIONS;
use futures::try_join;
use std::process::exit;
use taskrs_db::actions::errors::AlterUserError;
use taskrs_db::models::permission::dtos::PermissionCreate;
use taskrs_db::models::role::dtos::{Role, RoleCreate};
use taskrs_db::models::user::dtos::{User, UserCreate};
use taskrs_db::sea_orm::{ColumnTrait, Condition, ConnectionTrait, DbConn, DbErr};

const ROOT_ROLE_NAME: &str = "root";
const ROOT_ROLE_DESCRIPTION: &str = "Role which has every permission that is seeded at startup";

pub async fn seed_permissions(db: &DbConn) {
    info!("Seeding permissions from bundled permissions");

    let json_permissions: Vec<PermissionCreate> = serde_json::from_str(ALL_PERMISSIONS)
        .unwrap_or_else(|err| {
            error!("Error while deserializing embedded permissions: {}", err);
            exit(-1);
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
        error!("Database error while seeding permissions: {}", err);
        exit(-1);
    });
}

pub async fn seed_root_role(db: &DbConn) -> Role {
    info!("Seeding root role if it does not exist");

    let db_role = taskrs_db::actions::roles::get(
        None,
        Some(Condition::all().add(taskrs_db::models::role::Column::Name.eq(ROOT_ROLE_NAME))),
        db,
    )
    .await
    .unwrap_or_else(|err| {
        error!("Database error while getting root role: {}", err);
        exit(-1);
    });

    match db_role {
        None => taskrs_db::actions::roles::create(
            RoleCreate {
                name: ROOT_ROLE_NAME.to_string(),
                description: Some(ROOT_ROLE_DESCRIPTION.to_string()),
                ..Default::default()
            },
            db,
        )
        .await
        .unwrap_or_else(|err| {
            error!("Database error while inserting root role: {}", err);
            exit(-1);
        }),
        Some(role) => role,
    }
}

pub async fn seed_root_role_permissions(role_id: i32, db: &DbConn) {
    info!("Seeding permissions for root role");

    debug!("Getting all permissions");
    let all_permissions_future = taskrs_db::actions::permissions::get_all(None, None, db);
    let role_permissions_future =
        taskrs_db::actions::access_control::get_permission_of_role(role_id, db);

    let (all_permissions, role_permissions) =
        try_join!(all_permissions_future, role_permissions_future).unwrap_or_else(|err| {
            error!("Database error while getting permissions: {}", err);
            exit(-1);
        });

    let new_permission_ids: Vec<i32> = all_permissions
        .iter()
        .filter(|ap| role_permissions.iter().any(|rp| ap.id == rp.id))
        .map(|p| p.id)
        .collect();

    let old_permission_ids: Vec<i32> = role_permissions
        .iter()
        .filter(|rp| !all_permissions.iter().any(|ap| rp.id == ap.id))
        .map(|p| p.id)
        .collect();

    // Update permissions
    let grant_future =
        taskrs_db::actions::access_control::grant_role_permissions(role_id, new_permission_ids, db);
    let revoke_future = taskrs_db::actions::access_control::revoke_role_permissions(
        role_id,
        old_permission_ids,
        db,
    );

    try_join!(grant_future, revoke_future).unwrap_or_else(|err| {
        error!("Database error while updating permissions: {}", err);
        exit(-1);
    });
}

pub async fn seed_root_user(
    email: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
    db: &DbConn,
) -> User {
    info!("Seeding root user if it does not exist");

    let user = taskrs_db::actions::users::create(
        UserCreate {
            email: email.clone(),
            password,
            first_name,
            last_name,
            enabled: true,
            ..Default::default()
        },
        db,
    )
    .await;

    match user {
        Ok(user) => user,
        Err(err) => match err {
            AlterUserError::EmailExists => taskrs_db::actions::users::get(
                None,
                Some(Condition::all().add(taskrs_db::models::user::Column::Email.eq(email))),
                db,
            )
            .await
            .unwrap_or_else(|err| {
                error!("Database error while getting root user: {}", err);
                exit(-1);
            })
            .unwrap(),
            AlterUserError::Argon(err) => {
                error!("Error while hashing password for root user: {}", err);
                exit(-1);
            }
            AlterUserError::Db(err) => {
                error!("Database error while inserting root user: {}", err);
                exit(-1);
            }
        },
    }
}
