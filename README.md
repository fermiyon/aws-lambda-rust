![AWS](https://img.shields.io/badge/AWS-%23FF9900.svg?style=for-the-badge&logo=amazon-aws&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

By Selman Karaosmanoglu 

## Date created

8 July 2024

# Rust Cargo-Lambda Deployment on AWS Lambda Serverless

## Overview

This repository provides a guide for deploying a Rust-based AWS Lambda function on AWS Lambda
utilizing Cargo-Lambda Rust library 

## Architecture

```mermaid
flowchart LR;
    A((HTTPS invoke)) <-.-> B((AWS Lambda λ));
    B <--> C[main.rs];
```

## Key Advantages of Rust for AWS Lambda
- _Memory Efficiency_: Rust uses significantly less memory compared to Python. This is because Rust is a compiled language with efficient memory management, while Python is an interpreted language with higher memory overhead.
- _Performance_: Rust offers substantial performance improvements over Python, often ranging from 10 to 1000 times faster, depending on the task.
- _Deployment_: Rust enables small binary deployments. By optimizing for size, Rust binaries can be significantly smaller than those of other languages.
- _Cost Efficiency_: Building on ARM architecture can be up to 34% cheaper, especially with processors like Amazon’s Graviton.
- _Energy Efficiency_: Rust is highly energy-efficient, often comparable to C, and significantly more efficient than languages like Python.
- _Fast Response Times_: Rust can achieve very low response times, often as low as 2 milliseconds, due to its efficient execution and low-level control.

## Getting Started

## Install Rust

If you don't have Rust on your system install rust using https://rustup.rs

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install Cargo lambda

Cargo Lambda simplifies running, building, and deploying Rust functions on AWS Lambda without needing containers or VMs.

```bash
brew tap cargo-lambda/cargo-lambda
brew install cargo-lambda
```

For more information or other installation information on Cargo Lambda: https://www.cargo-lambda.info/guide/getting-started.html

## AWS CLI

### Make sure AWS CLI installed and configured.

If not, you can install with pip

```bash
pip install awscli
```

### Configure AWS

```bash
aws configure
```

## Testing locally

### Run the Rust app locally

```bash
make watch
```

![alt text](resources/1-make-watch.png)

### Test locally

```bash
make invoke
```

![alt text](resources/2-make-invoke.png)

## Deploy to AWS

```bash
make build && make deploy
```

![alt text](resources/3-make-build-deploy.png)

## Test deployed app

```bash
make aws-invoke
```

![alt text](resources/4-test.png)

### Lambda Function Summary

![alt text](resources/5-console-test-summary.png)

## Reference

* Rust AWS Lambda - O'Reiily 
