#!/bin/sh
dapr run --app-id mynode --app-port 3000 --port 3500 -- cargo run --example actix