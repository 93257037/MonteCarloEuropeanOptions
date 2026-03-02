import pandas as pd
import yfinance as yf
import numpy as np

#Adjusted close if no onda close 
def fetch_history(ticker: str, period: str = "1y", interval: str = "1d") -> pd.Series:
    data = yf.Ticker(ticker).history(period=period, interval=interval)
    if data.empty:
        raise ValueError(f"Nema istorijskih podataka za ticker {ticker}.")
    if "Adj Close" in data.columns:
        return data["Adj Close"].dropna()
    return data["Close"].dropna()


def get_current_price(ticker: str) -> float | None:
    t = yf.Ticker(ticker)

    #fast info za najnoviju cijenu ako nema onda last close 
    try:
        fast_info = getattr(t, "fast_info", None)
        if fast_info is not None:
            price = fast_info.get("last_price") or fast_info.get("last_close")
            if price is not None:
                return float(price)
    except Exception:
        pass

    # Fallback: poslednja dostupna cena iz kratke istorije
    try:
        hist = t.history(period="1d", interval="1m")
        if not hist.empty:
            col = "Adj Close" if "Adj Close" in hist.columns else "Close"
            return float(hist[col].dropna().iloc[-1])
    except Exception:
        pass

    return None


def estimate_annual_volatility(prices: pd.Series, trading_days: int = 252) -> float:
    """
    Procena godišnje volatilnosti na osnovu dnevnih log-prinosa.
    """
    log_returns = np.log(prices / prices.shift(1)).dropna()
    daily_vol = log_returns.std()
    return float(daily_vol * np.sqrt(trading_days))


