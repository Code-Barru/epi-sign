#!/bin/bash

DATE_FILE="/home/antoine/dockers/others/epi-sign/session_dates.txt"

TODAY=$(date +%d/%m/%y)

if grep -Fxq "$TODAY" "$DATE_FILE"; then
    echo "Today is an Epitech session, running cookie worker"
    ~/dockers/others/epi-sign/start-cookie-worker.sh
else
    echo "Today is not an Epitech session, going to sleep"
fi