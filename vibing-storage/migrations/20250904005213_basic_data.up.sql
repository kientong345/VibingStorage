-- Add up migration script here

INSERT INTO vibes (name, group_name)
VALUES
    ('spring', 'seasonal'),
    ('summer', 'seasonal'),
    ('autumn', 'seasonal'),
    ('winter', 'seasonal'),
    ('sunny', 'weather'),
    ('rainy', 'weather'),
    ('windy', 'weather'),
    ('cloudy', 'weather'),
    ('stormy', 'weather'),
    ('hotty', 'weather'),
    ('coldy', 'weather'),
    ('dawn', 'daytime'),
    ('morning', 'daytime'),
    ('noon', 'daytime'),
    ('afternoon', 'daytime'),
    ('dusk', 'daytime'),
    ('evening', 'daytime'),
    ('night', 'daytime')
;
