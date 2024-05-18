-- Inserts the same recipe two weeks in a row to enable lists to take surplus into account
INSERT INTO "day" ("recipe_id", "date") VALUES 
('1', '2023-11-09'),
('2', '2023-11-09'),
('1', '2023-11-16');