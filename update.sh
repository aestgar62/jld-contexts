#!/bin/sh
# Note: update src/lib.rs when updating URLs/filenames in this file.
cd "$(dirname "$0")" || exit 1
exec curl \
	https://www.w3.org/ns/did/v1 -o w3c-did-v1.jsonld \
	https://w3id.org/did-resolution/v1 -o w3c-did-resolution-v1.jsonld \
	-L
