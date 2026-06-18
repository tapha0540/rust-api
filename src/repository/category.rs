use sqlx::{MySql, Pool, QueryBuilder, mysql::MySqlQueryResult, query, query_as};

use crate::models::category::Category;

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn insert(
        pool: &Pool<MySql>,
        name: String,
        description: String,
        icon_url: String,
        parent_id: Option<i32>,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        query("INSERT INTO categories(name, description, parent_id, icon_url) VALUES (?,?,?,?)")
            .bind(name)
            .bind(description)
            .bind(parent_id)
            .bind(icon_url)
            .execute(pool)
            .await
    }

    pub async fn find_categories(pool: &Pool<MySql>) -> Result<Vec<Category>, sqlx::Error> {
        query_as::<MySql, Category>(
        "SELECT id, name, description, parent_id, icon_url, created_at, updated_at FROM categories",
    )
    .fetch_all(pool)
    .await
    }
    pub async fn find_category_by_id(pool: &Pool<MySql>, id: i32) -> Result<Category, sqlx::Error> {
        query_as::<MySql, Category>("SELECT id, name, description, parent_id, icon_url, created_at, updated_at FROM categories WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &Pool<MySql>, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        query("DELETE FROM categories WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
    }

    pub async fn update(
        pool: &Pool<MySql>,
        category: Category,
        id: i32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let mut query_builder = QueryBuilder::<MySql>::new("UPDATE categories SET ");

        let mut separated = query_builder.separated(", ");
        let mut has_fields = false;

        // IMPORTANT : On pousse "colonne = " ET on lie la valeur dans la même expression de séparation
        if let Some(name) = category.name {
            separated
                .push("name")
                .push_unseparated(" = ")
                .push_bind_unseparated(name);
            has_fields = true;
        }

        if let Some(description) = category.description {
            separated
                .push("description")
                .push_unseparated(" = ")
                .push_bind_unseparated(description);
            has_fields = true;
        }

        if let Some(parent_id) = category.parent_id {
            separated
                .push("parent_id")
                .push_unseparated(" = ")
                .push_bind_unseparated(parent_id);
            has_fields = true;
        }

        if let Some(icon_url) = category.icon_url {
            separated
                .push("icon_url")
                .push_unseparated(" = ")
                .push_bind_unseparated(icon_url);
            has_fields = true;
        }

        // On ferme explicitement le gestionnaire de séparation avant d'ajouter le WHERE
        drop(separated);

        if !has_fields {
            return Err(sqlx::Error::Protocol(
                "Aucun champ fourni pour la mise à jour".into(),
            ));
        }

        // Ajout de la clause WHERE avec les espaces nécessaires autour des mots-clés
        query_builder.push(" WHERE id = ").push_bind(id);

        let query = query_builder.build();
        query.execute(pool).await
    }
}
