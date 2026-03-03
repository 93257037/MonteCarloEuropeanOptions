use rand::distributions::Distribution;
use rand_distr::Normal;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::thread;
use std::time::Instant;

use num_cpus;

#[derive(Debug)]
struct Config {
    s0: f64,
    k: f64,
    r: f64,
    sigma: f64,
    t: f64,
    n_paths: usize,
    option_type: OptionType,
    n_threads: Option<usize>,
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
enum OptionType {
    Call,
    Put,
}

impl OptionType {
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "call" => Ok(Self::Call),
            "put" => Ok(Self::Put),
            other => Err(format!("Nepoznat tip opcije: {} (dozvoljeno: call, put)", other)),
        }
    }
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return Err("Očekuju se argumenti. Primer:\n  cargo run --release -- --s0 100 --k 110 --r 0.02 --sigma 0.3 --t 1.0 --paths 1000000 --option call --output ../data/rust_simulations.csv".into());
    }

    let mut s0 = None;
    let mut k = None;
    let mut r = None;
    let mut sigma = None;
    let mut t = None;
    let mut n_paths = None;
    let mut option_type = None;
    let mut n_threads: Option<usize> = None;
    let mut output: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--s0" => {
                i += 1;
                s0 = Some(args.get(i).ok_or("--s0 zahteva vrednost")?.parse().map_err(|_| "Neispravna vrednost za --s0")?);
            }
            "--k" => {
                i += 1;
                k = Some(args.get(i).ok_or("--k zahteva vrednost")?.parse().map_err(|_| "Neispravna vrednost za --k")?);
            }
            "--r" => {
                i += 1;
                r = Some(args.get(i).ok_or("--r zahteva vrednost")?.parse().map_err(|_| "Neispravna vrednost za --r")?);
            }
            "--sigma" => {
                i += 1;
                sigma = Some(args.get(i).ok_or("--sigma zahteva vrednost")?.parse().map_err(|_| "Neispravna vrednost za --sigma")?);
            }
            "--t" => {
                i += 1;
                t = Some(args.get(i).ok_or("--t zahteva vrednost")?.parse().map_err(|_| "Neispravna vrednost za --t")?);
            }
            "--paths" => {
                i += 1;
                n_paths = Some(args.get(i).ok_or("--paths zahteva vrednost")?.parse().map_err(|_| "Neispravna vrednost za --paths")?);
            }
            "--option" => {
                i += 1;
                let val = args.get(i).ok_or("--option zahteva vrednost")?;
                option_type = Some(OptionType::from_str(val)?);
            }
            "--output" => {
                i += 1;
                let val = args.get(i).ok_or("--output zahteva vrednost")?;
                output = Some(PathBuf::from(val));
            }
            "--threads" => {
                i += 1;
                let val = args.get(i).ok_or("--threads zahteva vrednost")?;
                let parsed: usize = val.parse().map_err(|_| "Neispravna vrednost za --threads")?;
                n_threads = Some(parsed);
            }
            other => {
                return Err(format!("arg unknown: {}", other));
            }
        }
        i += 1;
    }

    Ok(Config {
        s0: s0.ok_or("NO --s0")?,
        k: k.ok_or("No --k")?,
        r: r.ok_or("NO --r")?,
        sigma: sigma.ok_or("NO --sigma")?,
        t: t.ok_or("No --t")?,
        n_paths: n_paths.ok_or("No --paths")?,
        option_type: option_type.ok_or("No --option (call/put)")?,
        n_threads,
        output,
    })
}

fn simulate_terminal_prices(
    s0: f64,
    r: f64,
    sigma: f64,
    t: f64,
    n_paths: usize,
) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0).expect("Normal distribution");
    let drift = (r - 0.5 * sigma * sigma) * t;
    let vol = sigma * t.sqrt();

    let mut prices = Vec::with_capacity(n_paths);
    for _ in 0..n_paths {
        let z = normal.sample(&mut rng);
        let st = s0 * (drift + vol * z).exp();
        prices.push(st);
    }
    prices
}

fn european_payoff(st: f64, k: f64, option_type: OptionType) -> f64 {
    match option_type {
        OptionType::Call => (st - k).max(0.0),
        OptionType::Put => (k - st).max(0.0),
    }
}

fn price_european_option_mc(
    cfg: &Config,
) -> (f64, Vec<f64>, Vec<f64>) {
    let prices = simulate_terminal_prices(cfg.s0, cfg.r, cfg.sigma, cfg.t, cfg.n_paths);
    let mut payoffs = Vec::with_capacity(cfg.n_paths);
    for &st in &prices {
        payoffs.push(european_payoff(st, cfg.k, cfg.option_type));
    }
    let discount = (-cfg.r * cfg.t).exp();
    let mut sum_disc = 0.0;
    for &p in &payoffs {
        sum_disc += discount * p;
    }
    let price = sum_disc / cfg.n_paths as f64;
    (price, prices, payoffs)
}

fn price_european_option_mc_parallel(cfg: &Config, n_threads: usize) -> f64 {
    let n_threads = n_threads.max(1).min(cfg.n_paths);

    let paths_per_thread = cfg.n_paths / n_threads;
    let remainder = cfg.n_paths % n_threads;

    let mut handles = Vec::with_capacity(n_threads);

    for i in 0..n_threads {
        let paths_for_this = paths_per_thread + if i < remainder { 1 } else { 0 };
        if paths_for_this == 0 {
            continue;
        }

        let s0 = cfg.s0;
        let r = cfg.r;
        let sigma = cfg.sigma;
        let t = cfg.t;
        let k = cfg.k;
        let opt_type = cfg.option_type;

        let handle = thread::spawn(move || {
            let prices = simulate_terminal_prices(s0, r, sigma, t, paths_for_this);
            let discount = (-r * t).exp();
            let mut sum_disc = 0.0;
            for st in prices {
                let payoff = european_payoff(st, k, opt_type);
                sum_disc += discount * payoff;
            }
            sum_disc
        });

        handles.push(handle);
    }

    let mut total_disc = 0.0;
    let mut total_paths = 0usize;
    for (idx, h) in handles.into_iter().enumerate() {
        let partial = h.join().expect("thread panicked");
        total_disc += partial;
        // svaka nit ima paths_per_thread ili paths_per_thread+1
        let paths_for_thread = paths_per_thread + if idx < remainder { 1 } else { 0 };
        total_paths += paths_for_thread;
    }

    total_disc / total_paths as f64
}

fn write_results_csv(
    path: &PathBuf,
    prices: &[f64],
    payoffs: &[f64],
) -> Result<(), Box<dyn Error>> {
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    std::fs::create_dir_all(parent)?;
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(&["S_T", "payoff"])?;
    for (&st, &p) in prices.iter().zip(payoffs.iter()) {
        wtr.write_record(&[st.to_string(), p.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    let cfg = match parse_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Greška u argumentima: {e}");
            std::process::exit(1);
        }
    };

    println!("Rust Monte Carlo (sekvencijalno + paralelno)");
    println!("Parametri: {:?}", cfg);

    let start = Instant::now();
    let (price_seq, prices, payoffs) = price_european_option_mc(&cfg);
    let elapsed_seq = start.elapsed().as_secs_f64();

    println!(
        "Procijenjena cijena {:?} opcije (sekvencijalno): {:.6}",
        cfg.option_type, price_seq
    );
    println!("Vreme izvođenja (sekvencijalno): {:.6} sekundi", elapsed_seq);

    if let Some(path) = cfg.output.as_ref() {
        match write_results_csv(path, &prices, &payoffs) {
            Ok(_) => println!("Rezultati sačuvani u: {}", path.display()),
            Err(e) => eprintln!("Greška pri upisu CSV fajla: {e}"),
        }
    } else {
        // podrazumevani naziv koji jasno označava da je iz Rust implementacije
        let default_path = PathBuf::from("../data/rust_simulations_rust.csv");
        match write_results_csv(&default_path, &prices, &payoffs) {
            Ok(_) => println!(
                "Output fajl nije zadat; rezultati su sačuvani u podrazumevani RUST fajl: {}",
                default_path.display()
            ),
            Err(e) => eprintln!("Greška pri upisu podrazumevanog RUST CSV fajla: {e}"),
        }
    }

    // Paralelna verzija zasnovana na nitima
    let default_threads = num_cpus::get();
    let n_threads = cfg.n_threads.unwrap_or(default_threads);
    println!(
        "\nPokrećem paralelnu verziju sa {} niti (podrazumevano: {}).",
        n_threads, default_threads
    );

    let start_par = Instant::now();
    let price_par = price_european_option_mc_parallel(&cfg, n_threads);
    let elapsed_par = start_par.elapsed().as_secs_f64();

    println!(
        "Procijenjena cijena {:?} opcije (paralelno): {:.6}",
        cfg.option_type, price_par
    );
    println!(
        "Vreme izvođenja (paralelno, {} niti): {:.6} sekundi",
        n_threads, elapsed_par
    );

    if elapsed_par > 0.0 {
        let speedup = elapsed_seq / elapsed_par;
        let efficiency = speedup / n_threads as f64;
        println!("Speedup (t_seq / t_par) = {:.4}", speedup);
        println!("Efikasnost (speedup / broj niti) = {:.4}", efficiency);
    }
}

