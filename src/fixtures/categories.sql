-- Level 1 Categories
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '11111111-1111-1111-1111-111111111111',
  'Mobility',
  'mobility',
  NULL,
  1,
  1,
  TRUE,
  'Cars, Motorbikes, Bicycles',
  NOW()
),
(
  '22222222-2222-2222-2222-222222222222',
  'Real Estate',
  'real-estate',
  NULL,
  1,
  2,
  TRUE,
  'Rentals & Properties for Sale',
  NOW()
),
(
  '33333333-3333-3333-3333-333333333333',
  'Mobile Devices',
  'mobile-devices',
  NULL,
  1,
  3,
  TRUE,
  'Smartphones, Tablets, Accessories',
  NOW()
),
(
  '44444444-4444-4444-4444-444444444444',
  'Computing & Electronics',
  'computing-electronics',
  NULL,
  1,
  4,
  TRUE,
  'Laptops, Desktops, TVs, Cameras',
  NOW()
),
(
  '55555555-5555-5555-5555-555555555555',
  'Games',
  'games',
  NULL,
  1,
  5,
  TRUE,
  'Video Games, Consoles, Board Games',
  NOW()
),
(
  '66666666-6666-6666-6666-666666666666',
  'Home & Living',
  'home-living',
  NULL,
  1,
  6,
  TRUE,
  'Furniture, Appliances, Lighting',
  NOW()
),
(
  '77777777-7777-7777-7777-777777777777',
  'Fashion & Style',
  'fashion-style',
  NULL,
  1,
  7,
  TRUE,
  'Men, Women, Kids, Babies',
  NOW()
),
(
  '88888888-8888-8888-8888-888888888888',
  'Jewelry',
  'jewelry',
  NULL,
  1,
  8,
  TRUE,
  'Rings, Watches, Necklaces, Bracelets',
  NOW()
),
(
  '99999999-9999-9999-9999-999999999999',
  'Beauty & Personal Care',
  'beauty-personal-care',
  NULL,
  1,
  9,
  TRUE,
  'Hair, Skincare, Makeup, Fragrances',
  NOW()
),
(
  'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
  'Baby & Kids',
  'baby-kids',
  NULL,
  1,
  10,
  TRUE,
  'Clothing, Toys, Baby Accessories',
  NOW()
),
(
  'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb',
  'Services',
  'services',
  NULL,
  1,
  11,
  TRUE,
  'Home, Tech, Repair & Maintenance',
  NOW()
),
(
  'cccccccc-cccc-cccc-cccc-cccccccccccc',
  'Food & Agriculture',
  'food-agriculture',
  NULL,
  1,
  12,
  TRUE,
  'Fresh & Preserved Food, Farm Produce',
  NOW()
),
(
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  'Pets & Animals',
  'pets-animals',
  NULL,
  1,
  13,
  TRUE,
  'Pet Accessories, Food, Animals',
  NOW()
);
-- Level 2 - Mobility
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee',
  'Cars',
  'cars',
  '11111111-1111-1111-1111-111111111111',
  2,
  1,
  TRUE,
  'Used & New Cars',
  NOW()
),
(
  'ffffffff-ffff-ffff-ffff-ffffffffffff',
  'Motorbikes',
  'motorbikes',
  '11111111-1111-1111-1111-111111111111',
  2,
  2,
  TRUE,
  'Motorcycles & Scooters',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000001',
  'Bicycles',
  'bicycles',
  '11111111-1111-1111-1111-111111111111',
  2,
  3,
  TRUE,
  'Bikes & Cycling Gear',
  NOW()
);
-- Level 2 - Real Estate
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000002',
  'Rentals',
  'rentals',
  '22222222-2222-2222-2222-222222222222',
  2,
  1,
  TRUE,
  'Properties for Rent',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000003',
  'Properties for Sale',
  'properties-for-sale',
  '22222222-2222-2222-2222-222222222222',
  2,
  2,
  TRUE,
  'Properties for Purchase',
  NOW()
);
-- Level 3 - Rentals
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000004',
  'Houses',
  'houses-rent',
  '00000000-0000-0000-0000-000000000002',
  3,
  1,
  TRUE,
  'Residential Houses for Rent',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000005',
  'Apartments',
  'apartments-rent',
  '00000000-0000-0000-0000-000000000002',
  3,
  2,
  TRUE,
  'Flats & Apartments for Rent',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000006',
  'Hostels',
  'hostels-rent',
  '00000000-0000-0000-0000-000000000002',
  3,
  3,
  TRUE,
  'Student Hostels',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000007',
  'Offices',
  'offices-rent',
  '00000000-0000-0000-0000-000000000002',
  3,
  4,
  TRUE,
  'Commercial Office Space',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000008',
  'Shops',
  'shops-rent',
  '00000000-0000-0000-0000-000000000002',
  3,
  5,
  TRUE,
  'Retail Shops for Rent',
  NOW()
);
-- Level 3 - Properties for Sale
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000009',
  'Houses',
  'houses-sale',
  '00000000-0000-0000-0000-000000000003',
  3,
  1,
  TRUE,
  'Houses for Sale',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000010',
  'Buildings',
  'buildings-sale',
  '00000000-0000-0000-0000-000000000003',
  3,
  2,
  TRUE,
  'Commercial Buildings',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000011',
  'Apartments',
  'apartments-sale',
  '00000000-0000-0000-0000-000000000003',
  3,
  3,
  TRUE,
  'Apartments for Sale',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000012',
  'Shops',
  'shops-sale',
  '00000000-0000-0000-0000-000000000003',
  3,
  4,
  TRUE,
  'Shops for Sale',
  NOW()
);
-- Level 2 - Mobile Devices
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000013',
  'Smartphones',
  'smartphones',
  '33333333-3333-3333-3333-333333333333',
  2,
  1,
  TRUE,
  'iPhone, Samsung, Tecno, Infinix',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000014',
  'Tablets',
  'tablets',
  '33333333-3333-3333-3333-333333333333',
  2,
  2,
  TRUE,
  'iPads, Samsung Tabs, Huawei',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000015',
  'Mobile Accessories',
  'mobile-accessories',
  '33333333-3333-3333-3333-333333333333',
  2,
  3,
  TRUE,
  'Cases, Chargers, Screen Protectors',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000016',
  'Smartwatches',
  'smartwatches',
  '33333333-3333-3333-3333-333333333333',
  2,
  4,
  TRUE,
  'Apple Watch, Samsung Watch, Fitbit',
  NOW()
);
-- Level 2 - Computing & Electronics
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000017',
  'Laptops',
  'laptops',
  '44444444-4444-4444-4444-444444444444',
  2,
  1,
  TRUE,
  'MacBooks, Dell, HP, Lenovo',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000018',
  'Desktop Computers',
  'desktops',
  '44444444-4444-4444-4444-444444444444',
  2,
  2,
  TRUE,
  'PCs & All-in-Ones',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000019',
  'Monitors',
  'monitors',
  '44444444-4444-4444-4444-444444444444',
  2,
  3,
  TRUE,
  'Gaming & Professional Monitors',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000020',
  'Computer Components',
  'computer-components',
  '44444444-4444-4444-4444-444444444444',
  2,
  4,
  TRUE,
  'RAM, SSD, GPU, Motherboards',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000021',
  'Computer Accessories',
  'computer-accessories',
  '44444444-4444-4444-4444-444444444444',
  2,
  5,
  TRUE,
  'Keyboards, Mice, Headsets',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000022',
  'System Units',
  'system-units',
  '44444444-4444-4444-4444-444444444444',
  2,
  6,
  TRUE,
  'Pre-built & Custom PCs',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000023',
  'Televisions',
  'televisions',
  '44444444-4444-4444-4444-444444444444',
  2,
  7,
  TRUE,
  'LED, OLED, Smart TVs',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000024',
  'Audio Equipment',
  'audio-equipment',
  '44444444-4444-4444-4444-444444444444',
  2,
  8,
  TRUE,
  'Speakers, Soundbars, Amplifiers',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000025',
  'Headphones',
  'headphones',
  '44444444-4444-4444-4444-444444444444',
  2,
  9,
  TRUE,
  'Wireless & Wired Headphones',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000026',
  'Cameras',
  'cameras',
  '44444444-4444-4444-4444-444444444444',
  2,
  10,
  TRUE,
  'DSLR, Mirrorless, Action Cameras',
  NOW()
);
-- Level 2 - Games
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000027',
  'Video Games',
  'video-games',
  '55555555-5555-5555-5555-555555555555',
  2,
  1,
  TRUE,
  'PS5, Xbox, PC, Nintendo Games',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000028',
  'Consoles & Controllers',
  'consoles',
  '55555555-5555-5555-5555-555555555555',
  2,
  2,
  TRUE,
  'PS5, Xbox Series X, Switch',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000029',
  'Board & Card Games',
  'board-games',
  '55555555-5555-5555-5555-555555555555',
  2,
  3,
  TRUE,
  'Monopoly, Chess, Poker',
  NOW()
);
-- Level 2 - Home & Living
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000030',
  'Furniture',
  'furniture',
  '66666666-6666-6666-6666-666666666666',
  2,
  1,
  TRUE,
  'Sofas, Beds, Tables, Chairs',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000031',
  'Lighting',
  'lighting',
  '66666666-6666-6666-6666-666666666666',
  2,
  2,
  TRUE,
  'Lamps, Chandeliers, Bulbs',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000032',
  'Appliances',
  'appliances',
  '66666666-6666-6666-6666-666666666666',
  2,
  3,
  TRUE,
  'Kitchen & Home Appliances',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000033',
  'Cookware',
  'cookware',
  '66666666-6666-6666-6666-666666666666',
  2,
  4,
  TRUE,
  'Pots, Pans, Utensils',
  NOW()
);
-- Level 3 - Appliances
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000034',
  'Kitchen Appliances',
  'kitchen-appliances',
  '00000000-0000-0000-0000-000000000032',
  3,
  1,
  TRUE,
  'Microwaves, Blenders, Fridges',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000035',
  'Home Appliances',
  'home-appliances',
  '00000000-0000-0000-0000-000000000032',
  3,
  2,
  TRUE,
  'Washers, Dryers, Vacuum Cleaners',
  NOW()
);
-- Level 2 - Fashion & Style
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000036',
  'Men',
  'men-fashion',
  '77777777-7777-7777-7777-777777777777',
  2,
  1,
  TRUE,
  'Men Clothing, Footwear, Accessories',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000037',
  'Women',
  'women-fashion',
  '77777777-7777-7777-7777-777777777777',
  2,
  2,
  TRUE,
  'Women Clothing, Footwear, Accessories',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000038',
  'Kids',
  'kids-fashion',
  '77777777-7777-7777-7777-777777777777',
  2,
  3,
  TRUE,
  'Children Fashion (5-12 years)',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000039',
  'Babies',
  'babies-fashion',
  '77777777-7777-7777-7777-777777777777',
  2,
  4,
  TRUE,
  'Baby Fashion (0-4 years)',
  NOW()
);
-- Level 3 - Men
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000040',
  'Clothing',
  'men-clothing',
  '00000000-0000-0000-0000-000000000036',
  3,
  1,
  TRUE,
  'Shirts, Pants, Jackets',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000041',
  'Footwear',
  'men-footwear',
  '00000000-0000-0000-0000-000000000036',
  3,
  2,
  TRUE,
  'Shoes, Sneakers, Sandals',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000042',
  'Accessories',
  'men-accessories',
  '00000000-0000-0000-0000-000000000036',
  3,
  3,
  TRUE,
  'Bags, Belts, Hats',
  NOW()
);
-- Level 3 - Women
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000043',
  'Clothing',
  'women-clothing',
  '00000000-0000-0000-0000-000000000037',
  3,
  1,
  TRUE,
  'Dresses, Tops, Skirts',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000044',
  'Footwear',
  'women-footwear',
  '00000000-0000-0000-0000-000000000037',
  3,
  2,
  TRUE,
  'Heels, Flats, Boots',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000045',
  'Accessories',
  'women-accessories',
  '00000000-0000-0000-0000-000000000037',
  3,
  3,
  TRUE,
  'Bags, Scarves, Belts',
  NOW()
);
-- Level 3 - Kids
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000046',
  'Clothing',
  'kids-clothing',
  '00000000-0000-0000-0000-000000000038',
  3,
  1,
  TRUE,
  'Kids Apparel',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000047',
  'Footwear',
  'kids-footwear',
  '00000000-0000-0000-0000-000000000038',
  3,
  2,
  TRUE,
  'Kids Shoes',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000048',
  'Accessories',
  'kids-accessories',
  '00000000-0000-0000-0000-000000000038',
  3,
  3,
  TRUE,
  'Kids Bags, Hats',
  NOW()
);
-- Level 3 - Babies
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000049',
  'Clothing',
  'babies-clothing',
  '00000000-0000-0000-0000-000000000039',
  3,
  1,
  TRUE,
  'Baby Onesies, Sleepwear',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000050',
  'Footwear',
  'babies-footwear',
  '00000000-0000-0000-0000-000000000039',
  3,
  2,
  TRUE,
  'Baby Shoes',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000051',
  'Accessories',
  'babies-accessories',
  '00000000-0000-0000-0000-000000000039',
  3,
  3,
  TRUE,
  'Baby Bibs, Hats',
  NOW()
);
-- Level 2 - Jewelry
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000052',
  'Men',
  'men-jewelry',
  '88888888-8888-8888-8888-888888888888',
  2,
  1,
  TRUE,
  'Men Jewelry',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000053',
  'Women',
  'women-jewelry',
  '88888888-8888-8888-8888-888888888888',
  2,
  2,
  TRUE,
  'Women Jewelry',
  NOW()
);
-- Level 3 - Men Jewelry
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000054',
  'Rings & Earrings',
  'men-rings-earrings',
  '00000000-0000-0000-0000-000000000052',
  3,
  1,
  TRUE,
  'Men Rings & Earrings',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000055',
  'Watches',
  'men-watches',
  '00000000-0000-0000-0000-000000000052',
  3,
  2,
  TRUE,
  'Men Watches',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000056',
  'Necklaces',
  'men-necklaces',
  '00000000-0000-0000-0000-000000000052',
  3,
  3,
  TRUE,
  'Men Necklaces',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000057',
  'Bracelets',
  'men-bracelets',
  '00000000-0000-0000-0000-000000000052',
  3,
  4,
  TRUE,
  'Men Bracelets',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000058',
  'Cufflinks',
  'cufflinks',
  '00000000-0000-0000-0000-000000000052',
  3,
  5,
  TRUE,
  'Cufflinks',
  NOW()
);
-- Level 3 - Women Jewelry
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000059',
  'Rings & Earrings',
  'women-rings-earrings',
  '00000000-0000-0000-0000-000000000053',
  3,
  1,
  TRUE,
  'Women Rings & Earrings',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000060',
  'Watches',
  'women-watches',
  '00000000-0000-0000-0000-000000000053',
  3,
  2,
  TRUE,
  'Women Watches',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000061',
  'Necklaces',
  'women-necklaces',
  '00000000-0000-0000-0000-000000000053',
  3,
  3,
  TRUE,
  'Women Necklaces',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000062',
  'Beads',
  'beads',
  '00000000-0000-0000-0000-000000000053',
  3,
  4,
  TRUE,
  'Beaded Jewelry',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000063',
  'Bracelets & Bangles',
  'bracelets-bangles',
  '00000000-0000-0000-0000-000000000053',
  3,
  5,
  TRUE,
  'Bracelets & Bangles',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000064',
  'Anklets',
  'anklets',
  '00000000-0000-0000-0000-000000000053',
  3,
  6,
  TRUE,
  'Anklets',
  NOW()
);
-- Level 2 - Beauty & Personal Care
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000065',
  'Hair & Oral Care',
  'hair-oral-care',
  '99999999-9999-9999-9999-999999999999',
  2,
  1,
  TRUE,
  'Shampoo, Conditioner, Toothpaste',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000066',
  'Skincare',
  'skincare',
  '99999999-9999-9999-9999-999999999999',
  2,
  2,
  TRUE,
  'Moisturizers, Serums, Cleansers',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000067',
  'Fragrances',
  'fragrances',
  '99999999-9999-9999-9999-999999999999',
  2,
  3,
  TRUE,
  'Perfumes, Colognes',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000068',
  'Makeup',
  'makeup',
  '99999999-9999-9999-9999-999999999999',
  2,
  4,
  TRUE,
  'Foundation, Lipstick, Mascara',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000069',
  'Intimate Wellness',
  'intimate-wellness',
  '99999999-9999-9999-9999-999999999999',
  2,
  5,
  TRUE,
  'Personal Care Products',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000070',
  'Beauty Accessories',
  'beauty-accessories',
  '99999999-9999-9999-9999-999999999999',
  2,
  6,
  TRUE,
  'Brushes, Mirrors, Tools',
  NOW()
);
-- Level 2 - Baby & Kids
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000071',
  'Fashion',
  'baby-fashion',
  'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
  2,
  1,
  TRUE,
  'Baby & Kids Clothing',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000072',
  'Games & Toys',
  'toys',
  'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
  2,
  2,
  TRUE,
  'Educational & Fun Toys',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000073',
  'Baby Food & Nutrition',
  'baby-food',
  'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
  2,
  3,
  TRUE,
  'Formula, Baby Cereal',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000074',
  'Baby Accessories',
  'baby-accessories',
  'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
  2,
  4,
  TRUE,
  'Diapers, Wipes, Bottles',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000075',
  'Playground & Outdoor',
  'playground',
  'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
  2,
  5,
  TRUE,
  'Swings, Slides, Outdoor Toys',
  NOW()
);
-- Level 2 - Services
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000076',
  'Home Services',
  'home-services',
  'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb',
  2,
  1,
  TRUE,
  'Cleaning, Plumbing, Electrical',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000077',
  'Tech Services',
  'tech-services',
  'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb',
  2,
  2,
  TRUE,
  'IT Support, Web Development',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000078',
  'Repair & Maintenance',
  'repair-services',
  'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb',
  2,
  3,
  TRUE,
  'Phone, Computer, Appliance Repair',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000079',
  'Other Services',
  'other-services',
  'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb',
  2,
  4,
  TRUE,
  'Consulting, Tutoring, Events',
  NOW()
);
-- Level 2 - Food & Agriculture
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000080',
  'Preserved Food',
  'preserved-food',
  'cccccccc-cccc-cccc-cccc-cccccccccccc',
  2,
  1,
  TRUE,
  'Canned, Dried, Packaged',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000081',
  'Fresh Food',
  'fresh-food',
  'cccccccc-cccc-cccc-cccc-cccccccccccc',
  2,
  2,
  TRUE,
  'Fruits, Vegetables, Meat',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000082',
  'Farm Produce',
  'farm-produce',
  'cccccccc-cccc-cccc-cccc-cccccccccccc',
  2,
  3,
  TRUE,
  'Crops, Livestock, Eggs',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000083',
  'Agricultural Supplies',
  'agri-supplies',
  'cccccccc-cccc-cccc-cccc-cccccccccccc',
  2,
  4,
  TRUE,
  'Fertilizers, Tools, Equipment',
  NOW()
);
-- Level 2 - Pets & Animals
INSERT INTO categories(
  id,
  name,
  slug,
  parent_id,
  level,
  display_order,
  is_active,
  description,
  created_at
)
VALUES(
  '00000000-0000-0000-0000-000000000084',
  'Pet Accessories',
  'pet-accessories',
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  2,
  1,
  TRUE,
  'Collars, Beds, Carriers',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000085',
  'Pet Food & Feed',
  'pet-food',
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  2,
  2,
  TRUE,
  'Dog, Cat, Bird Food',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000086',
  'Cats',
  'cats',
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  2,
  3,
  TRUE,
  'Cats for Sale',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000087',
  'Dogs',
  'dogs',
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  2,
  4,
  TRUE,
  'Dogs for Sale',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000088',
  'Fish',
  'fish',
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  2,
  5,
  TRUE,
  'Aquarium Fish',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000089',
  'Birds',
  'birds',
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  2,
  6,
  TRUE,
  'Parrots, Finches, Chickens',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000090',
  'Rabbits',
  'rabbits',
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  2,
  7,
  TRUE,
  'Rabbits for Sale',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000091',
  'Other Animals',
  'other-animals',
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  2,
  8,
  TRUE,
  'Hamsters, Guinea Pigs, Reptiles',
  NOW()
);
SELECT
  'Categories inserted successfully!' AS message;
SELECT
  COUNT(*) AS total_categories
FROM
  categories;
