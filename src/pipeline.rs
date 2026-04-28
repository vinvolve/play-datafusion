use datafusion::prelude::*;
use datafusion::execution::context::SessionConfig;
use datafusion::error::Result;
use crate::config::PipelineConfig;
use datafusion::dataframe::DataFrameWriteOptions;

pub struct TradingPipeline{
    ctx: SessionContext,
    config: PipelineConfig,

}

impl TradingPipeline {
    pub fn new(config: PipelineConfig) -> Self {
        let df_config = SessionConfig::new()
            .with_target_partitions(config.partitions)
            .with_batch_size(8192);

        let ctx = SessionContext::new_with_config(df_config);
        Self { ctx, config }
    }


    // task 1
    pub async fn register_src(&self) -> Result<()> {
        println!("---Registering source---");
        self.ctx.register_csv(
            "trades",
            &self.config.trades_csv_path,
            CsvReadOptions::new()
        ).await?;
        self.ctx.register_json(
            "metadata",
            &self.config.metadata_json_path,
            JsonReadOptions::default()
        ).await?;

        Ok(())
    } 

    //task 2
    pub async fn detet_anomalies(&self) -> Result<(DataFrame)> {
        println!("---Running anomaly detection---");
        let sql = "
            WITH trade_metrics AS (
                SELECT ts_offset, symbol, price, volume,
                    AVG(price) OVER (PARTITION BY symbol ORDER BY ts_offset ROWS BETWEEN 4 PRECEDING AND CURRENT ROW) as moving_avg,
                    price - LAG(price, 1) OVER (PARTITION BY symbol ORDER BY ts_offset) as price_change
                FROM trades
            ),
            anomalies AS (
                SELECT * FROM trade_metrics WHERE ABS(price_change) > (moving_avg * 0.05)
            )
            SELECT a.ts_offset, a.symbol, m.sector, m.risk_tier, a.price, ROUND(a.moving_avg, 2) as moving_avg
            FROM anomalies a JOIN metadata m ON a.symbol = m.symbol
        ";
        self.ctx.sql(sql).await
        
    }

    //task 3
    pub async fn export_result(&self, df:DataFrame) -> Result<()> {
        println!("---Exporting result---");
        df.write_parquet(&self.config.output_dir, DataFrameWriteOptions::new(), None).await?;
        Ok(())


    }

    //entire pipeline
    pub async fn execute(&self) -> Result<()> {
        self.register_src().await?;
        let df = self.detet_anomalies().await?;
        println!("Preview of anomalies");
        df.clone().show().await?;
        self.export_result(df).await?;
        Ok(())
    }

}


