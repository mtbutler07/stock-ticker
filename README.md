# Stock Ticker

## Description

Simple Stock CLI tool, written in Rust, to pull quote data from Finnhub.io for a user provided stock symbol and display that information to the screen.

## Usage

1. Get an API Token from [finnhub](https://finnhub.io/)
2. Store the API token as environment variable `export FINNHUB_API_TOKEN=12345678`

## TODO

- [] Improve Error Handling for User Input
- [] Improve formatting when displaying quote information
- [] Leverage .query() for building url instead of format macro
- [] Improve usage documentation
