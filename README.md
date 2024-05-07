# Knapsack Solver

This is a simple Rust project for solving the 0/1 Knapsack problem.

## Problem Description

The 0/1 Knapsack problem is a classic optimization problem where the goal is to select a subset of items, each with a weight and a value, to maximize the total value while staying within a given weight capacity.

## Features

- Iterator for generating all possible combinations of items.
- Selection of the most profitable knapsack based on monetary value.

## Installation

### For Linux

#### Red Hat based distributions

1. Download the latest rpm file from the release page, e.g.

```shell
wget https://github.com/vasigorc/Knapsack-Solver/releases/download/v0.0.1/knapsack_problem-0.0.1-1.x86_64.rpm
```

2. Install the file

```shel
sudo rpm -i knapsack_problem-0.0.1-1.x86_64.rpm
# test installation
which /usr/bin/knapsack_problem
# returns /usr/bin/knapsack_problem
```

## Runbook

```shell
# get help page
/usr/bin/knapsack_problem cli -h
Usage: knapsack_problem cli --clocks-file <FILE> --weight <FLOAT>

Options:
      --clocks-file <FILE>
  -w, --weight <FLOAT>
  -h, --help                Print help

# run the binary
/usr/bin/knapsack_problem cli --clocks-file sample_clocks.json -w 5.0
The best Knapsack combination for the provided input is Knapsack { contents: [Clock { weight: 2.0, price: 5.0 }, Clock { weight: 3.0, price: 8.0 }], capacity: 5 }
```

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
