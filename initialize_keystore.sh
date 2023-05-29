#!/bin/bash

helpFunction()
{
   echo ""
   echo "Usage: $0 <base-path> <node-key-file>"
   echo -e "\t base-path: Base path to be used for the node with trailing '/'"
   echo -e "\t node-key-file Path to the node-key-file"
   echo ""
   exit 1 # Exit script after printing help
}

# Allow for '-v' to be used for checking the script version
while getopts "v" option; do
   case $option in
      v) echo v1.0.0 && exit;;
      *) echo "Unrecognized flag" ;;
   esac
done

# Print helpFunction in case parameters are empty
if [ -z "$1" ] || [ -z "$2" ]
then
   echo "Unexpected number of arguments, empty arguments are not valid";
   helpFunction
fi

keystore="$1chains/mediator_testnet/keystore/"
nodeKeyFile="$2"

mkdir -p "$keystore" && \
SEED=$( cat "$nodeKeyFile" | awk '{print "\x220x" $1 "\x22"}' ) && \
AURA=61757261 && \
SR_PUB=$( cat "$nodeKeyFile" | xargs -I {} ./subkey inspect --scheme sr25519 '0x{}' | sed -n 's/  Public key (hex):  0x//p' ) && \
( echo "$SEED"|tr -d '\n' ) > "$keystore$AURA$SR_PUB"

ls "$keystore"