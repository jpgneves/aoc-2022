cat input | awk 'BEGIN { max=0; current = 0; } { if (NF == 0) { if (current > max) { max = current } current = 0 } else { current += $1; } } END { print max }'
