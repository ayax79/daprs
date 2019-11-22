#!/bin/sh
dapr run --app-id actix-subscriber-example --app-port 3000 -- cargo run --example actix-subscriber --features="actix"