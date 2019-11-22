#!/bin/sh
topic="foo"
if [ $1 ] 
then
    topic=$1
fi
dapr publish --topic $topic --payload '{"message": "Message for the foo topic"}'