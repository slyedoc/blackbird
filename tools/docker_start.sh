#!/bin/bash

cd /app/

# Start the first process
litefs mount &

# Start the second process
./blackbird &

# Wait for any process to exit
wait -n

# Exit with status of process that exited first
exit $?