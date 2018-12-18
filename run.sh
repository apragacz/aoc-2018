#!/bin/bash

set -euox pipefail

main() {
    local src_filename="$1"
    local bin_filename
    shift
    bin_filename="${src_filename%.*}.bin"
    rustc "${src_filename}" -o "${bin_filename}"
    "./${bin_filename}" "$@"
}

main "$@"
