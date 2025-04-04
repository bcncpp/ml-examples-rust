
use std::error::Error;
use polars::prelude::*;

/** 
use rand::{seq::SliceRandom, thread_rng};

fn split_train_test(df: &DataFrame, train_frac: f64) -> PolarsResult<(DataFrame, DataFrame)> {
    let n_rows = df.height();
    let mut indices: Vec<usize> = (0..n_rows).collect();
    let mut rng = thread_rng();
    indices.shuffle(&mut rng);
    let split_idx = (train_frac * n_rows as f64).round() as usize;
    let (train_idx, test_idx) = indices.split_at(split_idx);
    let train_df = df.select(train_idx)?;
    let test_df = df.select(test_idx)?;
    Ok((train_df, test_df))
}
**/


fn main() -> Result<(), Box<dyn Error>> { 
    let q = LazyCsvReader::new("Advertising.csv").finish()?;
    let df = q.collect()?;
    println!("{}", df);
    /* 
    let features = array.slice(s![.., 0..3]).to_owned(); // First 3 columns
    let targets = array.column(3).to_owned(); // 4th column is Sales

    let dataset = Dataset::new(features, targets);

    // Fit the linear regression model
    let model = LinearRegression::default().fit(&dataset)?;

    let predictions = model.predict(&dataset);

    println!("Coefficients: {:?}", model.params());
    println!(
        "R² score: {:.4}",
        predictions.r2(&dataset).unwrap_or(f64::NAN)
    );
    */
    

    Ok(())
}

