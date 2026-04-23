-- Seed regions first
INSERT INTO regions(
  id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  'Greater Accra',
  1,
  NOW()
),
(
  gen_random_uuid(),
  'Ashanti',
  2,
  NOW()
),
(
  gen_random_uuid(),
  'Western',
  3,
  NOW()
),
(
  gen_random_uuid(),
  'Central',
  4,
  NOW()
),
(
  gen_random_uuid(),
  'Eastern',
  5,
  NOW()
),
(
  gen_random_uuid(),
  'Northern',
  6,
  NOW()
),
(
  gen_random_uuid(),
  'Volta',
  7,
  NOW()
),
(
  gen_random_uuid(),
  'Upper East',
  8,
  NOW()
),
(
  gen_random_uuid(),
  'Upper West',
  9,
  NOW()
),
(
  gen_random_uuid(),
  'Bono',
  10,
  NOW()
),
(
  gen_random_uuid(),
  'Ahafo',
  11,
  NOW()
),
(
  gen_random_uuid(),
  'Bono East',
  12,
  NOW()
),
(
  gen_random_uuid(),
  'Oti',
  13,
  NOW()
),
(
  gen_random_uuid(),
  'North East',
  14,
  NOW()
),
(
  gen_random_uuid(),
  'Savannah',
  15,
  NOW()
),
(
  gen_random_uuid(),
  'Western North',
  16,
  NOW()
);
-- Get region IDs and insert towns
DO $$DECLARE greater_accra_id UUID;
ashanti_id UUID;
western_id UUID;
central_id UUID;
eastern_id UUID;
northern_id UUID;
volta_id UUID;
upper_east_id UUID;
upper_west_id UUID;
bono_id UUID;
ahafo_id UUID;
bono_east_id UUID;
oti_id UUID;
north_east_id UUID;
savannah_id UUID;
western_north_id UUID;
BEGIN-- Get region IDs
SELECT
  id INTO greater_accra_id
FROM
  regions
WHERE
  name = 'Greater Accra';
SELECT
  id INTO ashanti_id
FROM
  regions
WHERE
  name = 'Ashanti';
SELECT
  id INTO western_id
FROM
  regions
WHERE
  name = 'Western';
SELECT
  id INTO central_id
FROM
  regions
WHERE
  name = 'Central';
SELECT
  id INTO eastern_id
FROM
  regions
WHERE
  name = 'Eastern';
SELECT
  id INTO northern_id
FROM
  regions
WHERE
  name = 'Northern';
SELECT
  id INTO volta_id
FROM
  regions
WHERE
  name = 'Volta';
SELECT
  id INTO upper_east_id
FROM
  regions
WHERE
  name = 'Upper East';
SELECT
  id INTO upper_west_id
FROM
  regions
WHERE
  name = 'Upper West';
SELECT
  id INTO bono_id
FROM
  regions
WHERE
  name = 'Bono';
SELECT
  id INTO ahafo_id
FROM
  regions
WHERE
  name = 'Ahafo';
SELECT
  id INTO bono_east_id
FROM
  regions
WHERE
  name = 'Bono East';
SELECT
  id INTO oti_id
FROM
  regions
WHERE
  name = 'Oti';
SELECT
  id INTO north_east_id
FROM
  regions
WHERE
  name = 'North East';
SELECT
  id INTO savannah_id
FROM
  regions
WHERE
  name = 'Savannah';
SELECT
  id INTO western_north_id
FROM
  regions
WHERE
  name = 'Western North';
-- Greater Accra towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  greater_accra_id,
  'Accra Central',
  1,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Airport Residential',
  2,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Achimota',
  3,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Adabraka',
  4,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Cantonments',
  5,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Dzorwulu',
  6,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'East Legon',
  7,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Kanda',
  8,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Kokomlemle',
  9,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Labone',
  10,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Lapaz',
  11,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Madina',
  12,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Osu',
  13,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Ring Road Central',
  14,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Roman Ridge',
  15,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Spintex',
  16,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Tema',
  17,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Teshie',
  18,
  NOW()
),
(
  gen_random_uuid(),
  greater_accra_id,
  'Nungua',
  19,
  NOW()
);
-- Ashanti towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  ashanti_id,
  'Kumasi Central',
  1,
  NOW()
),
(
  gen_random_uuid(),
  ashanti_id,
  'Adum',
  2,
  NOW()
),
(
  gen_random_uuid(),
  ashanti_id,
  'Asafo',
  3,
  NOW()
),
(
  gen_random_uuid(),
  ashanti_id,
  'Asokwa',
  4,
  NOW()
),
(
  gen_random_uuid(),
  ashanti_id,
  'Bantama',
  5,
  NOW()
),
(
  gen_random_uuid(),
  ashanti_id,
  'Bekwai',
  6,
  NOW()
),
(
  gen_random_uuid(),
  ashanti_id,
  'Mampong',
  7,
  NOW()
),
(
  gen_random_uuid(),
  ashanti_id,
  'Obuasi',
  8,
  NOW()
),
(
  gen_random_uuid(),
  ashanti_id,
  'Suame',
  9,
  NOW()
),
(
  gen_random_uuid(),
  ashanti_id,
  'Tafo',
  10,
  NOW()
);
-- Western towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  western_id,
  'Sekondi',
  1,
  NOW()
),
(
  gen_random_uuid(),
  western_id,
  'Takoradi',
  2,
  NOW()
),
(
  gen_random_uuid(),
  western_id,
  'Axim',
  3,
  NOW()
),
(
  gen_random_uuid(),
  western_id,
  'Effia',
  4,
  NOW()
),
(
  gen_random_uuid(),
  western_id,
  'Kwesimintsim',
  5,
  NOW()
),
(
  gen_random_uuid(),
  western_id,
  'Shama',
  6,
  NOW()
);
-- Central towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  central_id,
  'Cape Coast',
  1,
  NOW()
),
(
  gen_random_uuid(),
  central_id,
  'Winneba',
  2,
  NOW()
),
(
  gen_random_uuid(),
  central_id,
  'Kasoa',
  3,
  NOW()
),
(
  gen_random_uuid(),
  central_id,
  'Mankessim',
  4,
  NOW()
),
(
  gen_random_uuid(),
  central_id,
  'Elmina',
  5,
  NOW()
);
-- Eastern towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  eastern_id,
  'Koforidua',
  1,
  NOW()
),
(
  gen_random_uuid(),
  eastern_id,
  'Nkawkaw',
  2,
  NOW()
),
(
  gen_random_uuid(),
  eastern_id,
  'Akim Oda',
  3,
  NOW()
),
(
  gen_random_uuid(),
  eastern_id,
  'Suhum',
  4,
  NOW()
),
(
  gen_random_uuid(),
  eastern_id,
  'Nsawam',
  5,
  NOW()
);
-- Northern towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  northern_id,
  'Tamale',
  1,
  NOW()
),
(
  gen_random_uuid(),
  northern_id,
  'Yendi',
  2,
  NOW()
),
(
  gen_random_uuid(),
  northern_id,
  'Savelugu',
  3,
  NOW()
),
(
  gen_random_uuid(),
  northern_id,
  'Bimbilla',
  4,
  NOW()
);
-- Volta towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  volta_id,
  'Ho',
  1,
  NOW()
),
(
  gen_random_uuid(),
  volta_id,
  'Hohoe',
  2,
  NOW()
),
(
  gen_random_uuid(),
  volta_id,
  'Keta',
  3,
  NOW()
),
(
  gen_random_uuid(),
  volta_id,
  'Kpando',
  4,
  NOW()
);
-- Upper East towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  upper_east_id,
  'Bolgatanga',
  1,
  NOW()
),
(
  gen_random_uuid(),
  upper_east_id,
  'Navrongo',
  2,
  NOW()
),
(
  gen_random_uuid(),
  upper_east_id,
  'Bawku',
  3,
  NOW()
);
-- Upper West towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  upper_west_id,
  'Wa',
  1,
  NOW()
),
(
  gen_random_uuid(),
  upper_west_id,
  'Jirapa',
  2,
  NOW()
),
(
  gen_random_uuid(),
  upper_west_id,
  'Lawra',
  3,
  NOW()
);
-- Bono towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  bono_id,
  'Sunyani',
  1,
  NOW()
),
(
  gen_random_uuid(),
  bono_id,
  'Berekum',
  2,
  NOW()
),
(
  gen_random_uuid(),
  bono_id,
  'Techiman',
  3,
  NOW()
),
(
  gen_random_uuid(),
  bono_id,
  'Dormaa Ahenkro',
  4,
  NOW()
);
-- Ahafo towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  ahafo_id,
  'Goaso',
  1,
  NOW()
),
(
  gen_random_uuid(),
  ahafo_id,
  'Kenyasi',
  2,
  NOW()
),
(
  gen_random_uuid(),
  ahafo_id,
  'Bechem',
  3,
  NOW()
);
-- Bono East towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  bono_east_id,
  'Techiman',
  1,
  NOW()
),
(
  gen_random_uuid(),
  bono_east_id,
  'Kintampo',
  2,
  NOW()
),
(
  gen_random_uuid(),
  bono_east_id,
  'Atebubu',
  3,
  NOW()
),
(
  gen_random_uuid(),
  bono_east_id,
  'Nkoranza',
  4,
  NOW()
);
-- Oti towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  oti_id,
  'Dambai',
  1,
  NOW()
),
(
  gen_random_uuid(),
  oti_id,
  'Krachi',
  2,
  NOW()
),
(
  gen_random_uuid(),
  oti_id,
  'Jasikan',
  3,
  NOW()
);
-- North East towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  north_east_id,
  'Nalerigu',
  1,
  NOW()
),
(
  gen_random_uuid(),
  north_east_id,
  'Gambaga',
  2,
  NOW()
),
(
  gen_random_uuid(),
  north_east_id,
  'Walewale',
  3,
  NOW()
);
-- Savannah towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  savannah_id,
  'Damongo',
  1,
  NOW()
),
(
  gen_random_uuid(),
  savannah_id,
  'Sawla',
  2,
  NOW()
),
(
  gen_random_uuid(),
  savannah_id,
  'Salaga',
  3,
  NOW()
);
-- Western North towns
INSERT INTO towns(
  id,
  region_id,
  name,
  display_order,
  created_at
)
VALUES(
  gen_random_uuid(),
  western_north_id,
  'Sefwi Wiawso',
  1,
  NOW()
),
(
  gen_random_uuid(),
  western_north_id,
  'Bibiani',
  2,
  NOW()
),
(
  gen_random_uuid(),
  western_north_id,
  'Juaboso',
  3,
  NOW()
);
RAISE NOTICE 'Locations seeded successfully!';

END $$;
