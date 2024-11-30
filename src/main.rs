mod models;
mod config;
mod extraction;
mod transformation;
mod loading;

use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Carrega configurações
    let config = config::Config::load()
        .context("Falha ao carregar configurações")?;

    // Configura pool de conexão com o banco de dados
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("Falha ao conectar com o banco de dados")?;

    info!("Iniciando processo de ETL para criptomoedas");

    // Pipeline ETL
    match extraction::extract_crypto_data(&config).await {
        Ok(raw_data) => {
            let transformed_data = transformation::transform_crypto_data(raw_data)
                .context("Falha na transformação dos dados")?;

            match loading::load_to_postgres(&pool, &transformed_data).await {
                Ok(_) => info!("Processo de ETL concluído com sucesso!"),
                Err(e) => {
                    error!("Erro no carregamento dos dados: {}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            error!("Erro na extração dos dados: {}", e);
            return Err(e);
        }
    }

    Ok(())
}