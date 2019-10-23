# Daprs - Rust support for Microsoft Dapr

## Current Features
1. StateClient provides support for Dapr state service

## Examples

# Prerequisites
1. Install Dapr 
2. Install rust

### Actix Microservice Dapr State Example
This example uses a simple actix microservice to save and get state using the daprs::state::StateClient
location: examples/actix-state

Running the Example:
```
scripts/actix-state-run.sh

# In a seperate terminal
scripts/actix-state-post.sh
scripts/actix-state-get.sh
```
