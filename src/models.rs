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
    pub id: i32,
    pub name: String,
    pub url: String,
    pub logo_url: Option<String>,
    pub admins: Vec<String>,
    pub categories: Vec<String>,
    pub rules: String,
    pub description: String,
    pub registration_status: Registration,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CategoryDB {
    pub id: i32,
    pub name: String,
    pub servers: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub servers: Vec<Server>,
}

impl CategoryDB {
    async fn getByID(id: i32, pgPool: &PgPool) -> anyhow::Result<CategoryDB> {
        let category = sqlx::query_as!(CategoryDB, "SELECT * FROM categories WHERE id = $1", id)
            .fetch_optional(pgPool).await?.unwrap();
        Ok(category)
    }
    pub async fn getCategory(self, pgPool: &PgPool) -> anyhow::Result<Category> {
        let mut servers = vec![];
        for server_id in self.servers {
            let server = sqlx::query_as!(Server, r#"SELECT id, name as "name!: String", url as "url!: String", logo_url, admins as "admins!: Vec<String>", categories as "categories!: Vec<String>", rules as "rules!: String", description as "description!: String", registration_status as "registration_status!: Registration" FROM servers WHERE id = $1"#, server_id)
                .fetch_optional(pgPool).await?.unwrap();

            servers.push(server);
        }
        Ok(Category {
            id: self.id,
            name: self.name,
            servers,
        })
    }
}
