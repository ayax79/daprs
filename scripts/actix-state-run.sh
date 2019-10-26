#!/bin/sh
dapr run --app-id actix-state-example --app-port 3000 --port 3500 -- cargo run --example actix-state