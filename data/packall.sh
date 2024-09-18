#!/bin/bash
#
# Build MessagePack binaries from JSON
#
set -e

NEED=`which realpath`
if [[ "" == "${NEED}" ]]; then
        echo "Need to have \"realpath\" installed"
        exit 1
fi

SCRIPT=`realpath -s $0`
SDIR=`dirname ${SCRIPT}`
cd $SDIR

for F in master_decks hands_text
do
    rm -f ./bin/$F.msgpack
    ./pack_$F.ts
done

