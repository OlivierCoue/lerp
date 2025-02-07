#!/bin/bash

# Define an array of animation names
animations=("walk" "idle" "attack")

# Base paths
input_base="/mnt/c/Users/OlivierCoue/Documents/lerp/blender/renders"
output_base="/mnt/c/Users/OlivierCoue/Documents/lerp/blender/atlas"

# Loop over animations
for animation in "${animations[@]}"; do
    input_path="$input_base/$animation/*.png"
    output_path="$output_base/$animation.png"
    
    echo "Processing $animation..."
    
    montage "$input_path" -tile 16x8 -geometry +0+0 -background transparent -alpha on "$output_path"

    echo "Saved atlas: $output_path"
done

echo "All animations processed!"