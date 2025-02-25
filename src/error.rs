use sea_orm::DbErr;

pub type DbResult<T> = Result<T, DbErr>;
