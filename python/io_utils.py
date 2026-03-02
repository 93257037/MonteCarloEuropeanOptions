from pathlib import Path
from typing import Iterable

import pandas as pd


def save_simulation_results(
    output_dir: str,
    ticker: str,
    s_t: Iterable[float],
    payoffs: Iterable[float],
) -> Path:

    out_dir = Path(output_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    df = pd.DataFrame({"S_T": list(s_t), "payoff": list(payoffs)})
    file_path = out_dir / f"simulations_{ticker}.csv"
    df.to_csv(file_path, index=False)
    return file_path


def append_timing_result(
    output_file: str,
    language: str,
    impl_type: str,
    ticker: str,
    n_paths: int,
    n_workers: int,
    elapsed: float,
) -> Path:
    """
    Dodaje jedan red u CSV sa vremenima izvršavanja.
    Koristi se kasnije za eksperimente jakog/slabog skaliranja.
    """
    out_path = Path(output_file)
    out_path.parent.mkdir(parents=True, exist_ok=True)

    row = {
        "language": language,
        "implementation": impl_type,
        "ticker": ticker,
        "n_paths": n_paths,
        "n_workers": n_workers,
        "elapsed_sec": elapsed,
    }

    if out_path.exists():
        df = pd.read_csv(out_path)
        df = pd.concat([df, pd.DataFrame([row])], ignore_index=True)
    else:
        df = pd.DataFrame([row])

    df.to_csv(out_path, index=False)
    return out_path

