#!/bin/sh

# Posts an order id to a running version of the actix example
# Requires that the uuid util is installed an in the path
ORDER_ID=$(uuid)
JSON="{\"order_id\":\"${ORDER_ID}\", \"product\": \"Foo Widget\", \"quantity\": 5 }"

curl -vvvv -XPOST -d "$JSON" http://localhost:3500/v1.0/invoke/mynode/method/order