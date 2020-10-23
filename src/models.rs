use sqlx::postgres::PgPool;
use tokio::stream::StreamExt;

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
}

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub servers: Vec<Server>,
}

impl CategoryDB {
    async fn get_by_name(name: String, pg_pool: &PgPool) -> anyhow::Result<CategoryDB> {
        let category = sqlx::query_as!(CategoryDB, "SELECT * FROM categories WHERE name = $1", name)
            .fetch_optional(pg_pool).await?.unwrap();
        Ok(category)
    }
    pub async fn get_category(self, pg_pool: &PgPool) -> anyhow::Result<Category> {
        let mut servers = vec![];
        let mut servers_raw = sqlx::query!(r#"SELECT server_url as "server_url!: String" FROM servers_categories WHERE category_name = $1"#, &self.name)
            .fetch_many(pg_pool);
        while let Some(server_url_raw) = servers_raw.next().await {
            let server_url = server_url_raw?.right().unwrap().server_url;
            let server = sqlx::query_as!(Server, r#"SELECT name as "name!: String", url as "url!: String", server_name as "server_name!: String", logo_url, admins as "admins!: Vec<String>", categories as "categories!: Vec<String>", rules as "rules!: String", description as "description!: String", registration_status as "registration_status!: Registration" FROM servers WHERE url = $1"#, server_url)
                .fetch_optional(pg_pool).await?.unwrap();

            servers.push(server);
        }
        Ok(Category {
            name: self.name,
            servers,
        })
    }
}
