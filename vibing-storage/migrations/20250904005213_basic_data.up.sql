-- Add up migration script here

INSERT INTO vibe_groups (name)
VALUES
    ('seasonal'),
    ('weather'),
    ('daytime'),
    ('mood'),
    ('event')
;

INSERT INTO vibes (name, vibe_group)
VALUES
    ('spring', 1),
    ('summer', 1),
    ('autumn', 1),
    ('winter', 1),
    ('sunny', 2),
    ('rainy', 2),
    ('windy', 2),
    ('cloudy', 2),
    ('stormy', 2),
    ('hotty', 2),
    ('coldy', 2),
    ('dawn', 3),
    ('morning', 3),
    ('noon', 3),
    ('afternoon', 3),
    ('dusk', 3),
    ('evening', 3),
    ('night', 3)
;
