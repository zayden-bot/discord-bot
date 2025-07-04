UPDATE gambling
SET
    coins = coins - $2
WHERE
    id = $1;