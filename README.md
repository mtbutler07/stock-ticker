# Stock Ticker

## Description

Simple Stock CLI tool, written in Rust, to pull quote data from Finnhub.io for a user provided stock symbol and display that information to the screen.

## Usage

1. Get an API Token from [finnhub](https://finnhub.io/)
2. Store the API token as environment variable `export FINNHUB_API_TOKEN=12345678`

## TODO

- [x] Use serde to rename finnhub fields
- [] Improve Error Handling for User Input
- [x] Improve Error Handling when making API call
- [] Improve formatting when displaying quote information
- [x] Leverage .query() for building url query params instead of format macro
- [] Improve usage documentation
