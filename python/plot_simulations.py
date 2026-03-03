from __future__ import annotations

from pathlib import Path

import matplotlib.pyplot as plt
import pandas as pd


DATA_DIR = Path("../data")


def plot_price_distribution(ticker: str) -> None:
  
    file_path = DATA_DIR / f"simulations_{ticker}.csv"
    if not file_path.exists():
        print(f"No {file_path}, run sim.")
        return

    df = pd.read_csv(file_path)

    fig, axes = plt.subplots(1, 2, figsize=(12, 5))

    axes[0].hist(df["S_T"], bins=100, color="steelblue", alpha=0.8)
    axes[0].set_title(f"Distribucija završne cijene S_T ({ticker})")
    axes[0].set_xlabel("S_T")
    axes[0].set_ylabel("Frekvencija")

    axes[1].hist(df["payoff"], bins=100, color="darkorange", alpha=0.8)
    axes[1].set_title(f"Distribucija isplata opcije ({ticker})")
    axes[1].set_xlabel("Payoff")
    axes[1].set_ylabel("Frekvencija")

    plt.tight_layout()
    out_path = DATA_DIR / f"plot_distribution_{ticker}.png"
    plt.savefig(out_path, dpi=150)
    print(f"saved in : {out_path}")
    plt.close(fig)


def main() -> None:
    ticker = input("Unesi ticker za vizualizaciju: ").strip().upper()
    if not ticker:
        print("Ticker je obavezan.")
        return

    plot_price_distribution(ticker)


if __name__ == "__main__":
    main()

