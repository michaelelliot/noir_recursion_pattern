#!/usr/bin/env sh

TRANSCRIPT_FILE=".cache/transcript00.dat"
TRANSCRIPT_URL="https://aztec-ignition.s3.eu-west-2.amazonaws.com/MAIN+IGNITION/sealed/transcript00.dat"
EXPECTED_SIZE=322560348

# Check if the file exists and its size matches the expected size
if [ -f "$TRANSCRIPT_FILE" ]; then
    FILE_SIZE=$(wc -c <"$TRANSCRIPT_FILE" | tr -d ' ')
    if [ "$FILE_SIZE" -eq "$EXPECTED_SIZE" ]; then
        # Transcript file exists and is correct file size
        exit 0
    else
        echo "$TRANSCRIPT_FILE exists but is not the expected size."
    fi
fi

# If transcript file doesn't exist or size is incorrect, download it
echo "Downloading transcript to ${TRANSCRIPT_FILE}..."
mkdir -p .cache
curl -o "$TRANSCRIPT_FILE" "$TRANSCRIPT_URL"

echo "Download complete."
