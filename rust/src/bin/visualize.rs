use std::env;
use std::error::Error;
use std::path::PathBuf;

use csv::ReaderBuilder;
use plotters::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SimulationRow {
    #[serde(rename = "S_T")]
    s_t: f64,
    payoff: f64,
}

fn read_simulations(path: &PathBuf) -> Result<Vec<SimulationRow>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(path)?;
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        let row: SimulationRow = result?;
        rows.push(row);
    }
    Ok(rows)
}

fn plot_histogram(
    data: &[f64],
    title: &str,
    x_label: &str,
    output_path: &PathBuf,
    bins: usize,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min, max) = match (data.iter().cloned().fold(f64::INFINITY, f64::min),
                            data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)) {
        (a, b) if a.is_finite() && b.is_finite() && a < b => (a, b),
        _ => return Ok(()), // nema validnih podataka
    };

    let margin = (max - min) * 0.05;
    let x_min = min - margin;
    let x_max = max + margin;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption(title, ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, 0u32..1u32)?; // y max ćemo podesiti posle

    chart.configure_mesh().x_desc(x_label).y_desc("frequency").draw()?;

    // ručno pravljenje histogramskih binova
    let bin_width = (x_max - x_min) / bins as f64;
    let mut counts = vec![0u32; bins];
    for &v in data {
        if v < x_min || v > x_max {
            continue;
        }
        let idx = (((v - x_min) / bin_width) as usize).min(bins - 1);
        counts[idx] += 1;
    }
    let y_max = counts.iter().cloned().max().unwrap_or(0);
    if y_max == 0 {
        return Ok(());
    }

    // ponovo izgradimo chart sa pravim y opsegom
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption(title, ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, 0u32..y_max)?;

    chart.configure_mesh().x_desc(x_label).y_desc("frequency").draw()?;

    chart.draw_series(
        counts
            .iter()
            .enumerate()
            .map(|(i, &count)| {
                let x0 = x_min + i as f64 * bin_width;
                let x1 = x0 + bin_width;
                Rectangle::new(
                    [(x0, 0), (x1, count)],
                    BLUE.mix(0.6).filled(),
                )
            }),
    )?;

    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Upotreba:\n  cargo run --release --bin visualize -- <ticker> [putanja_do_csv]"
        );
        eprintln!(
            "Podrazumevana putanja CSV fajla: ../data/simulations_<TICKER>.csv\n\
             Izlazni PNG fajlovi biće u ../data/rust_plot_<TICKER>_*.png"
        );
        return Ok(());
    }

    let ticker = args[1].to_uppercase();
    let csv_path = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from(format!("../data/simulations_{}.csv", ticker))
    };

    if !csv_path.exists() {
        eprintln!("CSV fajl ne postoji: {}", csv_path.display());
        return Ok(());
    }

    let rows = read_simulations(&csv_path)?;
    if rows.is_empty() {
        eprintln!("CSV fajl je prazan: {}", csv_path.display());
        return Ok(());
    }

    let s_t_values: Vec<f64> = rows.iter().map(|r| r.s_t).collect();
    let payoff_values: Vec<f64> = rows.iter().map(|r| r.payoff).collect();

    let out_dir = PathBuf::from("../data");
    std::fs::create_dir_all(&out_dir)?;

    let price_path = out_dir.join(format!("rust_plot_{}_S_T.png", ticker));
    let payoff_path = out_dir.join(format!("rust_plot_{}_payoff.png", ticker));

    plot_histogram(
        &s_t_values,
        &format!("Distribucija završne cijene S_T ({}) [Rust]", ticker),
        "S_T",
        &price_path,
        50,
    )?;

    plot_histogram(
        &payoff_values,
        &format!("Distribucija isplata opcije ({}) [Rust]", ticker),
        "payoff",
        &payoff_path,
        50,
    )?;

    println!("Sačuvani grafikoni:");
    println!("  {}", price_path.display());
    println!("  {}", payoff_path.display());

    Ok(())
}

