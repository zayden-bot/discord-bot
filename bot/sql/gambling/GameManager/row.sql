SELECT
    g.id,
    g.coins,
    g.gems,

    COALESCE(l.level, 0) AS level,

    COALESCE(m.prestige, 0) AS prestige
FROM
    gambling AS g
LEFT JOIN
    levels AS l ON g.id = l.id
LEFT JOIN
    gambling_mine AS m ON g.id = m.id
WHERE
    g.id = $1;