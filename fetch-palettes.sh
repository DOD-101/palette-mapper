#! /usr/bin/env sh

# Script to pull in palettes from tinted-theming

set -e

cd "$(git rev-parse --show-toplevel)" || exit

pwd

if [ -d ".build" ]; then
    git -C .build pull
else
    git clone --depth 1 https://github.com/tinted-theming/schemes.git .build/schemes
fi

convert() {
    mkdir -p "palettes/$1"

    find "./.build/schemes/$1/" -name "*.yaml" | while read -r file; do
        base_name=$(basename "$file")
        name="${base_name%.yaml}"

        contents="$(grep "base..:" "$file" | sed 's/[ \t]*base..:[ \t]*\(.*\)"[ \t]*#*.*/\1"/' | jq -n -c '[inputs]')"

        echo "$contents" >"./palettes/$1/${name}.json"
    done
}

convert "base16"
convert "base24"

cp ./.build/schemes/LICENSE ./palettes/Tinted-Theming-LICENSE
