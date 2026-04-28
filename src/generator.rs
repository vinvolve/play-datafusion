use std::fs::File;
use std::io::{self, Write};


pub fn setup_mock_data(trades_path: &str, metadata_path: &str) -> io::Result<()> {
    let mut meta_file = File::create(metadata_path)?;
    let symbols = ["AAPL", "TSLA", "NVDA", "AMZN"];
    let sectors = ["Tech", "Auto", "Semiconductors", "Retail"];

    for i in 0..symbols.len() {
        writeln!(meta_file, "{{\"symbol\": \"{}\", \"sector\": \"{}\", \"risk_tier\": {}}}", 
            symbols[i], sectors[i], i % 3 + 1)?;
    }

    let mut trades_file = File::create(trades_path)?;
    writeln!(trades_file, "ts_offset,symbol,price,volume")?;

    let mut base_prices = [150.0, 200.0, 450.0, 130.0];
    for i in 1..=50_000 {
        let symbol_idx = i % 4;
        let walk = if i % 2 == 0 { 0.5 } else { -0.4 };
        base_prices[symbol_idx] += walk;
        writeln!(trades_file, "{},{},{:.2},{}", i, symbols[symbol_idx], base_prices[symbol_idx], (i % 100) * 10)?;
    }
    
    Ok(())






}
