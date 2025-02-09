# Crypto Profit Calculator

This Rust program reads cryptocurrency investment details from a `portfolio.txt` file, fetches live prices from the CoinGecko API, and calculates profits or losses. It then displays the results in a formatted table.

## Features

- Reads a portfolio of cryptocurrencies from a file.
- Fetches real-time prices using the [CoinGecko API](https://www.coingecko.com/en/api).
- Calculates profit or loss percentages and values.
- Displays the results in a neatly formatted table.

## How It Works

1. **Read Portfolio Data**  
   The program reads `portfolio.txt`, which contains cryptocurrency investments in the following format:

   ```
   SYMBOL BUY_PRICE AMOUNT
   ```

   Example:
   ```
   BAN 1.50 100
   DYM 2.30 50
   SCRT 5.00 200
   ```

2. **Fetch Live Prices**  
   It retrieves the latest USD prices for each cryptocurrency from CoinGecko.

3. **Calculate Profit/Loss**  
   The program compares the current price with the buy-in price and computes:
   - Profit/Loss percentage
   - Profit/Loss in USD

4. **Display Results in a Table**  
   Example Output:
   ```
   |---------------------------------------------------------------------------------|
   |                                  PORTFOLIO                                      |
   |---------------------------------------------------------------------------------|
   |  SYMBOL  | Capital(USD) | Entry Price | Current Price | Profit(%) | Profit(USD) |
   |---------------------------------------------------------------------------------|
   | BAN      | 100          | 1.50        | 1.80          | 20.00     | 20.00       |
   | DYM      | 50           | 2.30        | 2.10          | -8.70     | -4.35       |
   | SCRT     | 200          | 5.00        | 5.50          | 10.00     | 20.00       |
   |---------------------------------------------------------------------------------|
   | Total invested(USD): 350    | Profit(%): 10.43     | Profit(USD): 36.65       |
   |---------------------------------------------------------------------------------|
   ```

## Installation & Usage

### **1. Install Dependencies**
Ensure you have Rust installed. If not, install it via [rustup](https://rustup.rs/).

Clone this repository:
```sh
git clone https://github.com/jimtrung/profit-calculator.git
cd profit-calculator
```

### **2. Build & Run the Program**
```sh
cargo run
```

## How It Works Internally

- **`Crypto` Enum**: Defines supported cryptocurrencies.
- **`Info` Struct**: Stores buy price and investment amount.
- **`read_input()`**: Reads investment data from `portfolio.txt` and stores it in a HashMap.
- **`get_live_price()`**: Fetches live prices from the API.
- **`table()`**: Calculates profit/loss and prints a formatted report.
- **`main()`**: Runs the program.

## Cryptocurrency Basics

Cryptocurrency is a digital asset that uses cryptography for secure transactions. Unlike traditional money, cryptocurrencies are decentralized and often operate on blockchain networks.

### **Popular Cryptocurrencies**
- **Bitcoin (BTC)**: The first and most well-known cryptocurrency.
- **Ethereum (ETH)**: A blockchain platform supporting smart contracts.
- **Binance Coin (BNB)**: Used in the Binance ecosystem.
- **Solana (SOL)**: Known for its fast transaction speeds.

## License
This project is open-source under the MIT License.
