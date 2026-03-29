use chrono::{DateTime, Utc};
use diesel::{
    ExpressionMethods, QueryDsl, Queryable, Selectable,
    dsl::{delete, insert_into, now, update},
    prelude::{AsChangeset, Identifiable, Insertable},
    result::Error,
};

use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::schema::users;

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::users, check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub id: String,
    pub email: String,

    pub password_hash: String,
}
impl NewUser {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            id: Ulid::new().to_string(),
            email,
            password_hash,
        }
    }
    pub async fn insert_user(&self, conn: &mut AsyncPgConnection) -> Result<User, Error> {
        let user = insert_into(users::table)
            .values(self)
            .get_result(conn)
            .await?;
        Ok(user)
    }
}
#[derive(Debug, Selectable, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::users, check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub email: String,
    pub verified_at: Option<DateTime<Utc>>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn clean_up_users(conn: &mut AsyncPgConnection) -> Result<(), Error> {
        delete(users::table).execute(conn).await?;
        Ok(())
    }
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn get_user_by_email(email: &str) -> _ {
        let user = users::table.filter(users::email.eq(email));
        user
    }
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn get_user_by_id(id: &str) -> _ {
        let user = users::table.filter(users::id.eq(id));
        user
    }
    #[diesel::dsl::auto_type(no_type_alias)]

    pub async fn verify_user(id: &str) -> _ {
        update(users::table)
            .filter(users::id.eq(id))
            .set(users::verified_at.eq(now))
    }
}
