# Knapsack Solver

This is a simple Rust project for solving the 0/1 Knapsack problem.

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

#### Docker image

The Docker image used in the simulation is based on the buildpack-deps:22.04 image. It includes 
essential development tools and libraries, making it suitable for building and testing various software projects.

#### Prerequisites

- Docker installed on your local machine
- GitHub CLI (`gh`) installed
- The act tool installed (Install using: `gh extension install https://github.com/nektos/gh-act`)

#### Usage

To run the local GitHub Actions workflow simulation, use the provided script:

```bash
./run-gh-act.sh
```
