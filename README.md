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
