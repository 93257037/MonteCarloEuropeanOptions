from __future__ import annotations

import sys
import time
import multiprocessing as mp

from data_fetch import fetch_history, estimate_annual_volatility, get_current_price
from io_utils import save_simulation_results, append_timing_result
from monte_carlo import price_european_option_mc
from parallel_mc import price_european_option_mc_parallel


def ask_float(prompt: str, default: float | None = None) -> float:
    while True:
        raw = input(f"{prompt}" + (f" [{default}]" if default is not None else "") + ": ").strip()
        if not raw and default is not None:
            return default
        try:
            return float(raw.replace(",", "."))
        except ValueError:
            print("Nije broj.")


def ask_int(prompt: str, default: int | None = None) -> int:
    while True:
        raw = input(f"{prompt}" + (f" [{default}]" if default is not None else "") + ": ").strip()
        if not raw and default is not None:
            return default
        try:
            return int(raw)
        except ValueError:
            print("Mora biti cijeli broj.")


def main() -> None:
    print(" Monte Carlo European Options (Python – sequential + parallel) ")

    ticker = input("Ticker: ").strip().upper()
    if not ticker:
        print("Ticker je obavezan.")
        sys.exit(1)

    n_paths = ask_int("Broj putanja", default=100_000)

    
    try:
        prices = fetch_history(ticker)
    except Exception as e:
        print(f"Greška pri preuzimanju istorijskih podataka: {e}")
        sys.exit(1)

    s0 = float(prices.iloc[-1])
    sigma = estimate_annual_volatility(prices)

    live_price = get_current_price(ticker)

    print("\n Ticker info: ")
    print(f"Closeprice (S0) = {s0:.4f} USD")
    if live_price is not None:
        print(f"Real time ≈ {live_price:.4f} USD")
    else:
        print("Real time nije dostupna koristi se last close ")
    print(f"volatility sigma = {sigma:.4f}")

    r = ask_float("Risk-free r(dec)", default=0.02)
    print("Maturing time = T")
    t = ask_float("Unesi T u god", default=1.0)

    k_default = live_price if live_price is not None else s0
    k = ask_float("Strike price K", default=k_default)

    #When call and put 
    ref_price = live_price if live_price is not None else s0
    option_type = "call" if k >= ref_price else "put"
    print(f"Koja opcija se simulira: {option_type.upper()}")

    print("Starting (sekvencijalno)...")
    start = time.perf_counter()
    price, s_t, payoffs = price_european_option_mc(
        s0=s0,
        k=k,
        r=r,
        sigma=sigma,
        t=t,
        n_paths=n_paths,
        option_type=option_type,
    )
    elapsed_seq = time.perf_counter() - start

    print(f"\nProcijenjena cijena {option_type.upper()} opcije na {ticker}: {price:.4f}")
    print(f"Vreme izvođenja simulacije (sekvencijalno): {elapsed_seq:.6f} sekundi")

    timings_path = append_timing_result(
        output_file="../data/timings_python.csv",
        language="python",
        impl_type="sequential",
        ticker=ticker,
        n_paths=n_paths,
        n_workers=1,
        elapsed=elapsed_seq,
    )

    output_path = save_simulation_results(
        output_dir="../data",
        ticker=ticker,
        s_t=s_t,
        payoffs=payoffs,
    )
    print(f"file location: {output_path}")

    # PARALLEL IMPLEMENTATION (multiprocessing)
    n_workers_default = mp.cpu_count()
    n_workers = ask_int(
        f"Broj procesa za paralelnu verziju (default = broj jezgara = {n_workers_default})",
        default=n_workers_default,
    )

    print("Starting (paralelno, multiprocessing)...")
    start_par = time.perf_counter()
    price_par = price_european_option_mc_parallel(
        s0=s0,
        k=k,
        r=r,
        sigma=sigma,
        t=t,
        n_paths=n_paths,
        option_type=option_type,
        n_workers=n_workers,
    )
    elapsed_par = time.perf_counter() - start_par

    print(f"\nProcijenjena cijena {option_type.upper()} opcije (paralelno) na {ticker}: {price_par:.4f}")
    print(f"Vreme izvođenja simulacije (paralelno): {elapsed_par:.6f} sekundi")

    timings_path = append_timing_result(
        output_file="../data/timings_python.csv",
        language="python",
        impl_type="parallel_mp",
        ticker=ticker,
        n_paths=n_paths,
        n_workers=n_workers,
        elapsed=elapsed_par,
    )

    if elapsed_par > 0.0:
        speedup = elapsed_seq / elapsed_par
        efficiency = speedup / n_workers if n_workers > 0 else 0.0
        print(f"\nSpeedup (t_seq / t_par) = {speedup:.4f}")
        print(f"Efikasnost (speedup / broj procesa) = {efficiency:.4f}")

    print(f"\nTimings CSV (za jaku/slabu skalabilnost): {timings_path}")


if __name__ == "__main__":
    main()

