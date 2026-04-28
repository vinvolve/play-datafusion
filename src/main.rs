mod config;
mod generator;
mod pipeline;

use config::PipelineConfig;
use pipeline::TradingPipeline;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Booting Algorithmic Trading Pipeline ===\n");

    let config = PipelineConfig::default_local();

    generator::setup_mock_data(&config.trades_csv_path, &config.metadata_json_path)?;

    let pipeline = TradingPipeline::new(config);

    pipeline.execute().await?;

    println!("\n--- Pipeline Execution Completed Successfully ---");
    Ok(())
}
