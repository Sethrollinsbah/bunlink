use crate::models::user::User;

pub struct Team {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub last_seen: i64,
    pub admins: Vec<i64>,
    pub roles: Vec<TeamRole>,
    pub is_active: bool,
    pub ended_at: Option<i64>,
    pub status: TeamStatus,
}

 pub enum TeamStatus {
    Active,
    Inactive,
    Pending,
    Archived,
}

pub struct TeamRole {
    pub user_id: i64,
    pub role: String, // Or an enum representing roles
    pub permissions: Vec<String>, // Or an enum representing permissions
}

pub enum Role {
    Manager,
    User,
}
