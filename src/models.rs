#[derive(Queryable)]
pub struct Domain {
    pub id: i32,
    pub user_id: i32,
    pub hash: String,
    pub domain: String,
    pub flag: i32,
    pub state: i32,
    pub notes: String,
    pub create_time: i64,
}
