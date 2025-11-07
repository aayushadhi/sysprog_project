#!/usr/bin/env zsh

for i in {1..40}; do 
   dir="BRANCH$i"
   mkdir -p "$dir"

   file="$dir/branch_weekly_sales.txt"


   for day in {1..7}; do
      date=$(printf "2023-01-%02d" "$day")

      sales=$((RANDOM % 20 + 1))

      echo "$dir, PROD001, $sales, $date" >> "$file"

   done
done

