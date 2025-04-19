use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult)]
pub struct DomainOutput {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
}
