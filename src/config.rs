pub struct PipelineConfig {
    pub trades_csv_path: String,
    pub metadata_json_path: String,
    pub output_dir: String,
    pub partitions: usize,
}


impl PipelineConfig {
    pub fn default_local() -> Self {
        Self {
            trades_csv_path: "trades.csv".to_string(),
            metadata_json_path: "metadata.json".to_string(),
            output_dir: "anomaly_reports".to_string(),
            partitions: 8,

        }

    }

}
