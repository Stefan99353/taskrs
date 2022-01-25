use sea_orm::{DeriveColumn, EnumIter, IdenStatic};

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum QueryId {
    Id,
}
