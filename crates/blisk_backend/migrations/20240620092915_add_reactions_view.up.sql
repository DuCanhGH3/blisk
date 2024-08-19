CREATE OR REPLACE VIEW post_reactions_view AS (
    SELECT
        prt.post_id,
        prt.total,
        prt.like,
        prt.love,
        prt.laugh,
        prt.wow,
        prt.sad,
        prt.angry,
        ARRAY_AGG(prtop.col::PREACT) AS greatest
    FROM post_reactions_tally prt
    JOIN LATERAL (
        SELECT prtop.*
        FROM (
            VALUES
            (prt."like", 'like'), (prt."love", 'love'), (prt."laugh", 'laugh'),
            (prt."wow", 'wow'), (prt."sad", 'sad'), (prt."angry", 'angry')
        ) prtop (val, col)
        ORDER BY val DESC
        FETCH FIRST 3 ROW ONLY
    ) prtop ON TRUE
    GROUP BY prt.post_id, prt.total
);

CREATE OR REPLACE VIEW comment_reactions_view AS (
    SELECT
        crt.comment_id,
        crt.total,
        crt.like,
        crt.love,
        crt.laugh,
        crt.wow,
        crt.sad,
        crt.angry,
        ARRAY_AGG(crtop.col::PREACT) AS greatest
    FROM comment_reactions_tally crt
    JOIN LATERAL (
        SELECT crtop.*
        FROM (
            VALUES
            (crt."like", 'like'), (crt."love", 'love'), (crt."laugh", 'laugh'),
            (crt."wow", 'wow'), (crt."sad", 'sad'), (crt."angry", 'angry')
        ) crtop (val, col)
        ORDER BY val DESC
        FETCH FIRST 3 ROW ONLY
    ) crtop ON TRUE
    GROUP BY crt.comment_id, crt.total
);