use sqlx::postgres::PgPool;

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(rename = "registration", rename_all = "lowercase")]
pub enum Registration {
    Open,
    Invite,
    Closed,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Server {
    pub name: String,
    pub url: String,
    pub server_name: String,
    pub logo_url: Option<String>,
    pub admins: Vec<String>,
    pub categories: Vec<String>,
    pub rules: String,
    pub description: String,
    pub registration_status: Registration,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CategoryDB {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub description: Option<String>,
    pub servers: Vec<Server>,
}

impl CategoryDB {
    pub async fn get_all(pg_pool: &PgPool) -> anyhow::Result<Vec<Category>> {
        let categories = sqlx::query_as!(CategoryDB, "SELECT * FROM categories")
            .fetch_all(pg_pool)
            .await?;
        let mut categories_result = vec![];
        for category in categories {
            categories_result.push(category.get_category(pg_pool).await?);
        }

        Ok(categories_result)
    }
    pub async fn get_by_name(name: String, pg_pool: &PgPool) -> anyhow::Result<CategoryDB> {
        let category =
            sqlx::query_as!(CategoryDB, "SELECT * FROM categories WHERE name = $1", name)
                .fetch_optional(pg_pool)
                .await?
                .unwrap();
        Ok(category)
    }
    pub async fn get_category(self, pg_pool: &PgPool) -> anyhow::Result<Category> {
        let category = sqlx::query!(r#"SELECT description FROM categories WHERE name = $1"#, &self.name).fetch_one(pg_pool)
            .await?;
        let mut servers = vec![];
        let servers_raw = sqlx::query!(r#"SELECT server_url as "server_url!: String" FROM servers_categories WHERE category_name = $1"#, &self.name).fetch_all(pg_pool)
            .await?;
        for server_categories in servers_raw {
            let server_url = server_categories.server_url;
            let server = sqlx::query_as!(Server, r#"SELECT name as "name!: String", url as "url!: String", server_name as "server_name!: String", logo_url, admins as "admins!: Vec<String>", categories as "categories!: Vec<String>", rules as "rules!: String", description as "description!: String", registration_status as "registration_status!: Registration" FROM servers WHERE url = $1"#, server_url)
                .fetch_optional(pg_pool).await?.unwrap();

            servers.push(server);
        }
        Ok(Category {
            name: self.name,
            description: category.description,
            servers,
        })
    }
}
