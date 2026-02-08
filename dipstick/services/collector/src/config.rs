use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub collector: CollectorConfig,
    pub platforms: PlatformCredentials,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectorConfig {
    pub max_concurrent_collections: usize,
    pub default_check_frequency: u64,
    pub user_agent: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlatformCredentials {
    pub twitter: Option<TwitterCredentials>,
    pub meta: Option<MetaCredentials>,
    pub linkedin: Option<LinkedInCredentials>,
    pub youtube: Option<YouTubeCredentials>,
    pub bluesky: Option<BlueskyCredentials>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwitterCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub bearer_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetaCredentials {
    pub access_token: String,
    pub app_id: Option<String>,
    pub app_secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LinkedInCredentials {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YouTubeCredentials {
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlueskyCredentials {
    pub handle: String,
    pub app_password: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let config = Self {
            server: ServerConfig {
                port: env::var("COLLECTOR_PORT")
                    .unwrap_or_else(|_| "3001".to_string())
                    .parse()?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")?,
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()?,
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")?,
            },
            collector: CollectorConfig {
                max_concurrent_collections: env::var("MAX_CONCURRENT_COLLECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()?,
                default_check_frequency: env::var("DEFAULT_CHECK_FREQUENCY")
                    .unwrap_or_else(|_| "60".to_string())
                    .parse()?,
                user_agent: env::var("USER_AGENT").unwrap_or_else(|_| {
                    "NUJ Social Media Monitor/1.0 (https://nuj.org.uk; monitor@nuj.org.uk)".to_string()
                }),
            },
            platforms: PlatformCredentials {
                twitter: Self::get_twitter_creds(),
                meta: Self::get_meta_creds(),
                linkedin: Self::get_linkedin_creds(),
                youtube: Self::get_youtube_creds(),
                bluesky: Self::get_bluesky_creds(),
            },
        };

        Ok(config)
    }

    fn get_twitter_creds() -> Option<TwitterCredentials> {
        match (
            env::var("TWITTER_API_KEY").ok(),
            env::var("TWITTER_API_SECRET").ok(),
        ) {
            (Some(api_key), Some(api_secret)) => Some(TwitterCredentials {
                api_key,
                api_secret,
                bearer_token: env::var("TWITTER_BEARER_TOKEN").ok(),
            }),
            _ => None,
        }
    }

    fn get_meta_creds() -> Option<MetaCredentials> {
        env::var("META_ACCESS_TOKEN").ok().map(|access_token| MetaCredentials {
            access_token,
            app_id: env::var("META_APP_ID").ok(),
            app_secret: env::var("META_APP_SECRET").ok(),
        })
    }

    fn get_linkedin_creds() -> Option<LinkedInCredentials> {
        match (
            env::var("LINKEDIN_API_KEY").ok(),
            env::var("LINKEDIN_API_SECRET").ok(),
        ) {
            (Some(api_key), Some(api_secret)) => Some(LinkedInCredentials { api_key, api_secret }),
            _ => None,
        }
    }

    fn get_youtube_creds() -> Option<YouTubeCredentials> {
        env::var("YOUTUBE_API_KEY").ok().map(|api_key| YouTubeCredentials { api_key })
    }

    fn get_bluesky_creds() -> Option<BlueskyCredentials> {
        match (
            env::var("BLUESKY_HANDLE").ok(),
            env::var("BLUESKY_APP_PASSWORD").ok(),
        ) {
            (Some(handle), Some(app_password)) => Some(BlueskyCredentials { handle, app_password }),
            _ => None,
        }
    }
}
