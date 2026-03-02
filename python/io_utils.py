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

