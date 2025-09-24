use super::*;
use hyperlane_config::framework::database::*;
use tokio_postgres::NoTls;

impl DatabaseInitializer {
    pub async fn initialize() -> Result<(), DatabaseInitError> {
        let config = DatabaseConfig::from_env();

        // First, try to connect to the default postgres database to check if our target database exists
        Self::create_database_if_not_exists(&config).await?;

        // Initialize the connection pool
        initialize_global_pool(config.clone())
            .await
            .map_err(DatabaseInitError::PoolInitializationFailed)?;

        // Create tables if they don't exist
        Self::create_tables().await?;

        // Seed initial data
        Self::seed_initial_data().await?;

        // Perform health check
        global_health_check()
            .await
            .map_err(|e| DatabaseInitError::HealthCheckFailed(e))?;

        println!("Database initialization completed successfully");
        Ok(())
    }

    pub async fn create_database_if_not_exists(
        config: &DatabaseConfig,
    ) -> Result<(), DatabaseInitError> {
        // Connect to the default postgres database first
        let default_config = format!(
            "host={} port={} dbname=postgres user={} password={}",
            config.host, config.port, config.username, config.password
        );

        let (client, connection) = tokio_postgres::connect(&default_config, NoTls)
            .await
            .map_err(|e| DatabaseInitError::ConnectionFailed(e.to_string()))?;

        // Spawn the connection task
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });

        // Check if database exists
        let rows = client
            .query(
                "SELECT 1 FROM pg_database WHERE datname = $1",
                &[&config.database],
            )
            .await
            .map_err(|e| DatabaseInitError::QueryFailed(e.to_string()))?;

        if rows.is_empty() {
            // Database doesn't exist, create it
            let create_db_query = format!("CREATE DATABASE {}", config.database);
            client
                .simple_query(&create_db_query)
                .await
                .map_err(|e| DatabaseInitError::DatabaseCreationFailed(e.to_string()))?;

            println!("Database '{}' created successfully", config.database);
        } else {
            println!("Database '{}' already exists", config.database);
        }

        Ok(())
    }

    pub async fn create_tables() -> Result<(), DatabaseInitError> {
        let pool = get_global_pool().map_err(DatabaseInitError::PoolNotInitialized)?;

        let conn = pool
            .get_connection()
            .await
            .map_err(|e| DatabaseInitError::ConnectionAcquisitionFailed(e.to_string()))?;

        // Create users table
        let create_users_table = r#"
            CREATE TABLE IF NOT EXISTS users (
                id BIGSERIAL PRIMARY KEY,
                username VARCHAR(255) NOT NULL UNIQUE,
                password_hash VARCHAR(255) NOT NULL,
                email VARCHAR(255),
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                is_active BOOLEAN DEFAULT TRUE
            )
        "#;

        conn.simple_query(create_users_table)
            .await
            .map_err(|e| DatabaseInitError::TableCreationFailed(e.to_string()))?;

        // Create indexes
        let create_indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)",
            "CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)",
            "CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at)",
        ];

        for index_query in create_indexes {
            conn.simple_query(index_query)
                .await
                .map_err(|e| DatabaseInitError::IndexCreationFailed(e.to_string()))?;
        }

        // Create database version table for migrations
        let create_version_table = r#"
            CREATE TABLE IF NOT EXISTS database_version (
                id SERIAL PRIMARY KEY,
                version INTEGER NOT NULL,
                applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
        "#;

        conn.simple_query(create_version_table)
            .await
            .map_err(|e| DatabaseInitError::TableCreationFailed(e.to_string()))?;

        // Insert initial version if not exists
        let version_check = conn
            .query("SELECT COUNT(*) FROM database_version", &[])
            .await
            .map_err(|e| DatabaseInitError::QueryFailed(e.to_string()))?;

        let count: i64 = version_check[0].get(0);
        if count == 0 {
            conn.execute(
                "INSERT INTO database_version (version) VALUES ($1)",
                &[&1i32],
            )
            .await
            .map_err(|e| DatabaseInitError::QueryFailed(e.to_string()))?;
        }

        println!("Database tables created successfully");
        Ok(())
    }

    pub async fn seed_initial_data() -> Result<(), DatabaseInitError> {
        let pool = get_global_pool().map_err(DatabaseInitError::PoolNotInitialized)?;

        let conn = pool
            .get_connection()
            .await
            .map_err(|e| DatabaseInitError::ConnectionAcquisitionFailed(e.to_string()))?;

        // Check if root user already exists
        let existing_user = conn
            .query("SELECT id FROM users WHERE username = $1", &[&"root"])
            .await
            .map_err(|e| DatabaseInitError::QueryFailed(e.to_string()))?;

        if existing_user.is_empty() {
            // Hash the default password
            let password_hash = bcrypt::hash("hyperlane", bcrypt::DEFAULT_COST)
                .map_err(|e| DatabaseInitError::PasswordHashingFailed(e.to_string()))?;

            // Insert root user
            conn.execute(
                r#"
                INSERT INTO users (username, password_hash, email, is_active)
                VALUES ($1, $2, $3, $4)
                "#,
                &[&"root", &password_hash, &"root@hyperlane.local", &true],
            )
            .await
            .map_err(|e| DatabaseInitError::DataSeedingFailed(e.to_string()))?;

            println!(
                "Root user created successfully with username 'root' and password 'hyperlane'"
            );
        } else {
            println!("Root user already exists, skipping creation");
        }

        Ok(())
    }
}
