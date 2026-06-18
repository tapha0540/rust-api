use sqlx::{MySql, Pool, QueryBuilder, mysql::MySqlQueryResult, query, query_as};

use crate::models::user::{User, UserRole};

pub struct UserRepository;

impl UserRepository {
    pub async fn insert(
        pool: &Pool<MySql>,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        role: &UserRole,
        phone: String,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        query("INSERT INTO users(first_name, last_name, email, password, role, phone) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(password)
        .bind(role.to_string())
        .bind(phone)
        .execute(pool).await
    }

    pub async fn find(
        pool: &Pool<MySql>,
        id: Option<i32>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<User, sqlx::Error> {
        // 1. Validate that at least one search criteria is provided upfront
        if id.is_none() && email.is_none() && phone.is_none() {
            return Err(sqlx::Error::Protocol(
                "id, email, and phone cannot all be None.".to_string(),
            ));
        }

        let mut query_builder = QueryBuilder::new(
            "SELECT id, first_name, last_name, email, password, role, phone, profile_url, created_at, updated_at FROM users WHERE ",
        );

        // 2. Create the separated builder for dynamic OR conditions
        let mut separated = query_builder.separated(" OR ");

        if let Some(user_id) = id {
            separated.push("id = ").push_bind_unseparated(user_id);
        }

        if let Some(user_email) = email {
            separated.push("email = ").push_bind_unseparated(user_email);
        }

        if let Some(user_phone) = phone {
            separated.push("phone = ").push_bind_unseparated(user_phone);
        }

        // 3. Drop the separated helper to release the borrow on query_builder
        drop(separated);

        // 4. Execute the query
        query_builder.build_query_as::<User>().fetch_one(pool).await
    }

    pub async fn find_all(pool: &Pool<MySql>) -> Result<Vec<User>, sqlx::Error> {
        query_as::<_, User>("SELECT id, first_name, last_name, email, password, role, phone, profile_url, created_at, updated_at FROM users")
        .fetch_all(pool)
        .await
    }
    pub async fn delete(pool: &Pool<MySql>, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
    }
    pub async fn update(
        pool: &Pool<MySql>,
        user: User,
        id: i32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let mut query_builder = QueryBuilder::<MySql>::new("UPDATE users SET");

        let mut separated = query_builder.separated(", ");
        let mut has_fields = false;

        if let Some(first_name) = user.first_name {
            separated
                .push("first_name = ")
                .push_bind_unseparated(first_name);
            has_fields = true;
        }

        if let Some(last_name) = user.last_name {
            separated
                .push("last_name = ")
                .push_bind_unseparated(last_name);
            has_fields = true;
        }
        if let Some(email) = user.email {
            separated.push("email = ").push_bind_unseparated(email);
            has_fields = true;
        }
        if let Some(password) = user.password {
            separated
                .push("password = ")
                .push_bind_unseparated(password);
            has_fields = true;
        }
        if let Some(role) = user.role {
            separated.push("role = ").push_bind_unseparated(role);
            has_fields = true;
        }
        if let Some(phone) = user.phone {
            separated.push("phone = ").push_bind_unseparated(phone);
            has_fields = true;
        }
        if let Some(profile_url) = user.profile_url {
            separated
                .push("profile_url = ")
                .push_bind_unseparated(profile_url);
            has_fields = true;
        }

        drop(separated);

        if !has_fields {
            return Err(sqlx::Error::Protocol(
                "Aucun champ fourni pour la mise à jour".into(),
            ));
        }

        query_builder.push(" WHERE id = ").push_bind(id);
        let query = query_builder.build();
        query.execute(pool).await
    }
}
