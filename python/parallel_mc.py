from __future__ import annotations

import math
import multiprocessing as mp
from typing import Tuple

import numpy as np

from monte_carlo import simulate_terminal_prices, european_payoff


def _worker_chunk(
    args: Tuple[float, float, float, float, int, float, str],
) -> Tuple[float, float, int]:
   
    s0, r, sigma, t, n_paths, k, option_type = args
    s_t = simulate_terminal_prices(s0, r, sigma, t, n_paths)
    payoffs = european_payoff(s_t, k, option_type=option_type)
    discounted = np.exp(-r * t) * payoffs
    total = float(discounted.sum())
    total_sq = float((discounted**2).sum())
    return total, total_sq, n_paths


def price_european_option_mc_parallel(
    s0: float,
    k: float,
    r: float,
    sigma: float,
    t: float,
    n_paths: int,
    option_type: str = "call",
    n_workers: int | None = None,
) -> float:
    
    if n_workers is None or n_workers <= 0:
        n_workers = mp.cpu_count()

    n_workers = min(n_workers, n_paths)

    base = n_paths // n_workers
    remainder = n_paths % n_workers

    chunks = []
    for i in range(n_workers):
        size = base + (1 if i < remainder else 0)
        if size == 0:
            continue
        chunks.append((s0, r, sigma, t, size, k, option_type))

    with mp.Pool(processes=len(chunks)) as pool:
        results = pool.map(_worker_chunk, chunks)

    total_disc = 0.0
    total_paths = 0
    for partial_sum, _partial_sq, count in results:
        total_disc += partial_sum
        total_paths += count

    return total_disc / total_paths

