# MonteCarloEuropeanOptions

Paralelno vrednovanje Evropskih opcija Monte Carlo metodom

Predmet: Napredne tehnike programiranja

Student: Kristijan Trnjanac

---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

# Uvod i motivacija

Monte Carlo metode predstavljaju osnovni alat u numeričkim finansijama kada analitičko rješenje nije jednostavno ili nije dostupno. Posebno su značajne kod vrednovanja Evropskih opcija, gdje se cijena finansijskog instrumenta dobija simulacijom velikog broja mogućih budućih scenarija kretanja cijene osnovne imovine.

U ovom projektu razmatra se problem numeričkog određivanja cijene evropskih call i put opcija nad akcijama sa američkog tržišta. Kretanje cijene modeluje se stohastičkim procesom, dok se krajnja vrijednost opcije procjenjuje statističkom obradom rezultata simulacija.

Zbog potrebe za velikim brojem simulacija, Monte Carlo pristup brzo postaje računski zahtjevan. Međutim, pošto su pojedinačne simulacione putanje nezavisne jedna od druge, problem je izuzetno pogodan za paralelno izvršavanje.

---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

# Predmet i obuhvat projekta


Projekat se bavi sledećim zadacima:

Numeričko vrednovanje evropskih opcija Monte Carlo metodom

Razvoj sekvencijalne verzije algoritma

Paralelizacija algoritma 

Implementacija rješenja u programskim jezicima Python i Rust

Kvantitativna analiza dobijenih ubrzanja

Grafički prikaz rezultata simulacija i performansi


---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


# Matematički model

Cijena osnovne imovine modeluje se pomoću geometrijskog Braunovog kretanja (GBM). Na osnovu simuliranih putanja cijene, određuje se isplata opcije na dan dospijeća, nakon čega se vrši diskontovanje i računanje očekivane vrijednosti.

Razmatrani instrumenti su:

Evropska call opcija

Evropska put opcija

Istorijski podaci sa američkog tržišta koriste se isključivo za procjenu parametara modela (volatilnost), a ne za predviđanje budućih cijena.


---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


# IMPLEMENTACIJA


Python rješenje
Sekvencijalna varijanta

Klasična Monte Carlo simulacija bez paralelizacije

Generisanje velikog broja nezavisnih putanja cijene

Izračunavanje isplate opcije na kraju vremenskog intervala

Služi kao referentna implementacija za provjeru korektnosti



# Paralelizovana varijanta

Biblioteka: multiprocessing

Rad se dijeli na više procesa, pri čemu svaki proces obrađuje dio simulacija

Rezultati se agregiraju u glavnom procesu



# Rust rješenje

# Sekvencijalna varijanta

Potpuna implementacija Monte Carlo algoritma u Rust-u

Eksplicitna kontrola nad iteracijama i memorijom



# Paralelizovana varijanta

Biblioteka: Rayon

Paralelizacija zasnovana na data-parallelism konceptu

Automatsko upravljanje nitima i raspodjelom posla


---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


# Vizualizacija rezultata

Grafički prikaz rezultata realizovan je u Rust okruženju korišćenjem biblioteke Plotters.

Vizualizuju se:

Stabilizacija procjene cijene opcije u odnosu na broj simulacija

Distribucija krajnjih cijena osnovne imovine

Grafici ubrzanja za sekvencijalne i paralelne verzije

Svi rezultati se izvoze u statičke grafičke fajlove (PNG format).


---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


# Analiza performansi


# Mjerni kriterijumi

Ukupno vrijeme izvršavanja

Ostvareno ubrzanje (speedup)

Efikasnost paralelizacije

Eksperimentalni scenariji

Poređenje sekvencijalne i paralelne Python implementacije

Poređenje sekvencijalne i paralelne Rust implementacije

Jako skaliranje – konstantan obim posla

Slabo skaliranje – obim posla raste sa brojem jezgara


---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


# Promjenljivi parametri

Broj Monte Carlo simulacija

Broj vremenskih koraka

Volatilnost

Rok dospijeća opcije


---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


# Testno okruženje

Eksperimenti su izvršeni na sljedećim konfiguracijama:

Konfiguracija A

Procesor: AMD Ryzen 5 4600H

Memorija: 16 GB RAM

Grafička karta: NVIDIA GTX 1660 Ti

Konfiguracija B

Procesor: Intel Core i5-14400KF

Memorija: 32 GB RAM

Grafička karta: NVIDIA RTX 5060 (8 GB)

Softver:

Python: 3.14

Rust: 1.92


---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


# PROŠIRENJE ZA DIPLOMSKI RAD



Procjena rizika pomoću Monte Carlo simulacija

U okviru diplomskog rada planirano je proširenje sistema za analizu finansijskog rizika portfoplija.

Value-at-Risk (VaR)

Numerička procjena maksimalnog očekivanog gubitka

Razmatranje različitih nivoa pouzdanosti

Monte Carlo generisanje distribucije gubitaka

Conditional Value-at-Risk (CVaR)

Analiza prosječnog gubitka u ekstremnim scenarijima

Fokus na rep distribucije

Veći računski zahtjevi i značaj paralelizacije


---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


# Literatura i alati

Monte Carlo simulacije u finansijama

Python multiprocessing dokumentacija

Rayon – Rust biblioteka za paralelizaciju

Plotters – biblioteka za vizualizaciju u Rust-u
