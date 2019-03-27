/// The state of domain
pub enum DomainState{
    Normal = 1,
    Stopped = 2,
    Paused = 3
}

pub enum DomainFlag{
    /// If flag contain 2,System will open statistics
    /// function for domain.
    Stat = 2
}

pub struct Domain{
    pub id:i32,
    pub user_id:i32,
    pub hash:String,
    pub domain:String,
    pub flag:i32,
    pub state:i32,
    pub create_time:i64,
    pub notes:String
}