# Knapsack Solver

This is a simple Rust library for solving the 0/1 Knapsack problem.

## Problem Description

The 0/1 Knapsack problem is a classic optimization problem where the goal is to select a subset of items, each with a weight and a value, to maximize the total value while staying within a given weight capacity.

## Features

- Iterator for generating all possible combinations of items.
- Selection of the most profitable knapsack based on monetary value.

## Development

### GitHub Action Local Testing

This repository includes a script (`run-gh-act.sh`) that facilitates local testing of GitHub 
Actions workflows using the `act` tool. The purpose of this script is to simulate GitHub Actions 
locally before pushing changes to the remote repository.

#### Usage

To run the local GitHub Actions workflow simulation, use the provided script:

```bash
./run-gh-act.sh
```
