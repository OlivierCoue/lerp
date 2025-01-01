#!/bin/bash

# find /mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable\ character/warrior/warrior_armed_walk/ -type f -name "*.png" -exec cp {} /mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_1/ \;
# find /mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable\ character/warrior/warrior_armed_idle/ -type f -name "*.png" -exec cp {} /mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/ \;

# Mapping directions to numbers
declare -A direction_map=(
    ["S"]="00"
    ["SSW"]="01"
    ["SW"]="02"
    ["SWW"]="03"
    ["W"]="04"
    ["NWW"]="05"
    ["NW"]="06"
    ["NNW"]="07"
    ["N"]="08"
    ["NNE"]="09"
    ["NE"]="10"
    ["NEE"]="11"
    ["E"]="12"
    ["SEE"]="13"
    ["SE"]="14"
    ["SSE"]="15"
)

# for file in /mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_1/*.png; do
#     # Extract the direction (e.g., SE or N)
#     direction=$(echo "$file" | awk -F'_' '{print $6}')
#     echo $direction
#     # Extract the last number (e.g., 5 from original file name)
#     last_number=$(echo "$file" | grep -oP '\d+(?=\.png$)')
    
#     # Map the direction to the corresponding two-digit number
#     new_number=${direction_map[$direction]}
    
#     # Create the new filename
#     new_filename="${new_number}_${last_number}.png"
    
#     # Rename the file
#     mv "$file" "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_1/$new_filename"
# done

for file in /mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/*.png; do
    # Extract the direction (e.g., SE or N)
    direction=$(echo "$file" | awk -F'_' '{print $6}')
    echo $direction
    
    # Map the direction to the corresponding two-digit number
    new_number=${direction_map[$direction]}
    
    # Rename the file
    cp "$file" "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/${new_number}_0.png"
    cp "$file" "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/${new_number}_1.png"
    cp "$file" "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/${new_number}_2.png"
    cp "$file" "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/${new_number}_3.png"
    cp "$file" "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/${new_number}_4.png"
    cp "$file" "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/${new_number}_5.png"
    cp "$file" "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/${new_number}_6.png"
    cp "$file" "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/${new_number}_7.png"
done


# montage "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_walk/*.png" -tile 8x16 -geometry +0+0 -background transparent -alpha on "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/output/output.png"
# montage "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/*.png" -tile 8x16 -geometry +0+0 -background transparent -alpha on "/mnt/c/Users/OlivierCoue/Downloads/assets/assets/playable_char_idle/output.png"