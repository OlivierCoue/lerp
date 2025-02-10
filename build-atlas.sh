#!/bin/bash

# Define characters
characters=("archer" "enemy")

# Define animations with their tile dimensions (width x height)
declare -A animations
animations=(
    ["walk"]="16x8"
    ["idle"]="16x8"
    ["attack"]="16x8"
    ["death"]="16x8"
    ["dead"]="1x8"
)

# Base paths
input_base="/mnt/c/Users/OlivierCoue/Documents/lerp/blender/renders"
output_base="/mnt/c/Users/OlivierCoue/Documents/lerp/blender/atlas"

# Loop over characters
for character in "${characters[@]}"; do
    echo "Processing character: $character"

    # Loop over animations
    for animation in "${!animations[@]}"; do
        tile_size="${animations[$animation]}"  # Get tile size for this animation
        input_path="$input_base/$character/$animation/*.png"
        output_path="$output_base/$character-$animation.png"

        echo "Processing $animation for $character with tile size $tile_size..."

        # Ensure output directory exists
        mkdir -p "$(dirname "$output_path")"

        montage "$input_path" -tile "$tile_size" -geometry +0+0 -background transparent -alpha on "$output_path"

        echo "Saved atlas: $output_path"
    done
done

echo "All characters and animations processed!"
