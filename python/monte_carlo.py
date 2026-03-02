import numpy as np


def simulate_terminal_prices(
    s0: float,
    r: float,
    sigma: float,
    t: float,
    n_paths: int,
) -> np.ndarray:
    """
    endprice koristeci GBM formulu

    S_T = S0 * exp((r - 0.5 * sigma^2) * T + sigma * sqrt(T) * Z)
    gde je Z ~ N(0, 1).
    """
    z = np.random.normal(0.0, 1.0, size=n_paths)
    drift = (r - 0.5 * sigma**2) * t
    diffusion = sigma * np.sqrt(t) * z
    s_t = s0 * np.exp(drift + diffusion)
    return s_t


def european_payoff(s_t: np.ndarray, k: float, option_type: str = "call") -> np.ndarray:
    option_type = option_type.lower()
    if option_type == "call":
        return np.maximum(s_t - k, 0.0)
    if option_type == "put":
        return np.maximum(k - s_t, 0.0)
    raise ValueError("option_type mora biti 'call' ili 'put'.")


def price_european_option_mc(
    s0: float,
    k: float,
    r: float,
    sigma: float,
    t: float,
    n_paths: int,
    option_type: str = "call",
) -> tuple[float, np.ndarray, np.ndarray]:
   
    s_t = simulate_terminal_prices(s0, r, sigma, t, n_paths)
    payoffs = european_payoff(s_t, k, option_type=option_type)
    discounted_payoffs = np.exp(-r * t) * payoffs
    price = float(discounted_payoffs.mean())
    return price, s_t, payoffs

