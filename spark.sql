-- DEPRECATED

CREATE TABLE IF NOT EXISTS UserDetails ( 
    name TEXT,
    email TEXT,
    planType TEXT,
    apiKey TEXT,
    expiryon DATE ,
    hits TEXT
);

CREATE TABLE IF NOT EXISTS UserKeyStatus (
  apikey TEXT,
  status TEXT
);

-- CREATE TABLE IF NOT EXISTS UserRequests (
--     plantype TEXT,
--     hits INTEGER,
--     -- refdate DATE,
--     apikey TEXT,
--     expiryon DATE, 
-- );

-- INSERT INTO UserRequests (plantype, hits, refdate, apikey, expiryon) 
-- VALUES 
-- ('Basic', 100, '2024-03-04', 'apikey1', CURRENT_DATE + INTERVAL '30 days');


-- INSERT INTO UserRequests (plantype, hits, refdate, apikey, joinedon) 
-- VALUES 
-- ('Enterprize', 40, CURRENT_DATE, 'RadhaKrishna', CURRENT_DATE),
-- ('Hobby', 30, CURRENT_DATE, 'abc123', CURRENT_DATE);