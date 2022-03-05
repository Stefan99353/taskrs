use crate::logic::{CreateModelTrait, DeleteModelTrait, ReadModelTrait};
use crate::models::permission::{Permission, PermissionCreate};
use crate::models::role::{Role, RoleCreate};
use crate::models::user::{User, UserCreate};
use crate::permissions::ALL_PERMISSIONS;
use futures::try_join;
use taskrs_db::sea_orm::{ColumnTrait, Condition, ConnectionTrait, DbConn, DbErr};

const ROOT_ROLE_NAME: &str = "root";
const ROOT_ROLE_DESCRIPTION: &str = "Role which has every permission that is seeded at startup";

#[instrument(name = "seed_permissions", level = "debug", skip_all)]
pub async fn seed_permissions(db: &DbConn) -> anyhow::Result<()> {
    debug!("Deserializing embedded permissions");
    let json_permissions: Vec<PermissionCreate> = serde_json::from_str(ALL_PERMISSIONS)?;

    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            debug!("Geting current permissions");
            let db_permissions = Permission::all(txn).await?;

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
            if !new_permissions.is_empty() {
                Permission::create_many(new_permissions, txn).await?;
            }

            // Remove old permissions
            debug!("Removing old permissions");
            Permission::delete_condition(
                Condition::all()
                    .add(taskrs_db::models::permission::Column::Id.is_in(old_permission_ids)),
                txn,
            )
            .await?;

            Ok(())
        })
    })
    .await?;

    Ok(())
}

#[instrument(name = "seed_root_role", level = "debug", skip_all)]
pub async fn seed_root_role(db: &DbConn) -> anyhow::Result<Role> {
    debug!("Checking for root role");
    let db_role = Role::find_one(
        Condition::all().add(taskrs_db::models::role::Column::Name.eq(ROOT_ROLE_NAME)),
        db,
    )
    .await?;

    let role = match db_role {
        None => {
            Role::create(
                RoleCreate {
                    name: ROOT_ROLE_NAME.to_string(),
                    description: Some(ROOT_ROLE_DESCRIPTION.to_string()),
                    ..Default::default()
                },
                db,
            )
            .await?
        }
        Some(role) => role,
    };

    Ok(role)
}

#[instrument(name = "seed_root_role_permissions", level = "debug", skip_all)]
pub async fn seed_root_role_permissions(role_id: i32, db: &DbConn) -> anyhow::Result<()> {
    debug!("Getting all permissions");
    let all_permissions_future = Permission::all(db);
    let role_permissions_future = Role::permissions(role_id, db);

    let (all_permissions, role_permissions) =
        try_join!(all_permissions_future, role_permissions_future)?;

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
    let grant_future = Role::grant_permissions(role_id, new_permission_ids, db);
    let revoke_future = Role::revoke_permissions(role_id, old_permission_ids, db);

    try_join!(grant_future, revoke_future)?;
    Ok(())
}

#[instrument(name = "seed_root_user", level = "debug", skip_all, fields (email = %email))]
pub async fn seed_root_user(
    email: String,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
    db: &DbConn,
) -> anyhow::Result<User> {
    debug!("Check if user already exists");
    let db_user = User::find_one(
        Condition::all().add(taskrs_db::models::user::Column::Email.eq(email.clone())),
        db,
    )
    .await?;

    if let Some(user) = db_user {
        return Ok(user);
    }

    debug!("Creating root user");
    let user_create = UserCreate {
        email,
        password,
        first_name,
        last_name,
        enabled: true,
        ..Default::default()
    };

    debug!("Hashing password of user");
    let user_create = user_create.hash_password()?;

    Ok(User::create(user_create, db).await?)
}
