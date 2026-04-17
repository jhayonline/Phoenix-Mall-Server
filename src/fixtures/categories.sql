-- Clear existing categories
TRUNCATE categories CASCADE;
-- Insert categories
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
  'Electronics',
  'electronics',
  NULL,
  1,
  1,
  TRUE,
  'Laptops, Computers, Printers etc',
  NOW()
),
(
  '22222222-2222-2222-2222-222222222222',
  'Fashion',
  'fashion',
  NULL,
  1,
  2,
  TRUE,
  'Clothing & Accessories',
  NOW()
),
(
  '33333333-3333-3333-3333-333333333333',
  'Vehicles',
  'vehicles',
  NULL,
  1,
  3,
  TRUE,
  'Cars, Motorcycles & More',
  NOW()
),
(
  '44444444-4444-4444-4444-444444444444',
  'Home & Living',
  'home-living',
  NULL,
  1,
  4,
  TRUE,
  'Home & Living',
  NOW()
),
(
  '55555555-5555-5555-5555-555555555555',
  'Food & Agriculture',
  'food-agriculture',
  NULL,
  1,
  5,
  TRUE,
  'Fresh Food & Farm Products',
  NOW()
),
(
  '66666666-6666-6666-6666-666666666666',
  'Babies & Kids',
  'babies-kids',
  NULL,
  1,
  6,
  TRUE,
  'Baby Items & Kids Products',
  NOW()
),
(
  '77777777-7777-7777-7777-777777777777',
  'Beauty & Personal Care',
  'beauty-personal-care',
  NULL,
  1,
  7,
  TRUE,
  'Skincare & Cosmetics',
  NOW()
),
(
  '88888888-8888-8888-8888-888888888888',
  'Mobile Phones',
  'mobile-phones',
  '11111111-1111-1111-1111-111111111111',
  2,
  1,
  TRUE,
  'Mobile Phones, Tablets',
  NOW()
),
(
  '99999999-9999-9999-9999-999999999999',
  'Furniture',
  'furniture',
  '11111111-1111-1111-1111-111111111111',
  2,
  2,
  TRUE,
  'Home & Office Furniture',
  NOW()
),
(
  'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
  'Home Appliances',
  'home-appliances',
  '22222222-2222-2222-2222-222222222222',
  2,
  3,
  TRUE,
  'Kitchen & Home Appliances',
  NOW()
),
(
  'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb',
  'Jewelries',
  'jewelries',
  '22222222-2222-2222-2222-222222222222',
  2,
  4,
  TRUE,
  'Rings, Necklaces & More',
  NOW()
),
(
  'cccccccc-cccc-cccc-cccc-cccccccccccc',
  'Property',
  'property',
  '33333333-3333-3333-3333-333333333333',
  2,
  5,
  TRUE,
  'Houses & Real Estate',
  NOW()
),
(
  'dddddddd-dddd-dddd-dddd-dddddddddddd',
  'Services',
  'services',
  '33333333-3333-3333-3333-333333333333',
  2,
  6,
  TRUE,
  'Professional Services',
  NOW()
),
(
  'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee',
  'iPhone',
  'iphone',
  '88888888-8888-8888-8888-888888888888',
  3,
  1,
  TRUE,
  'Apple iPhones',
  NOW()
),
(
  'ffffffff-ffff-ffff-ffff-ffffffffffff',
  'Samsung',
  'samsung',
  '88888888-8888-8888-8888-888888888888',
  3,
  2,
  TRUE,
  'Samsung Galaxy Phones',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000001',
  'Tecno',
  'tecno',
  '88888888-8888-8888-8888-888888888888',
  3,
  3,
  TRUE,
  'Tecno Mobile Phones',
  NOW()
),
(
  '00000000-0000-0000-0000-000000000002',
  'Infinix',
  'infinix',
  '88888888-8888-8888-8888-888888888888',
  3,
  4,
  TRUE,
  'Infinix Mobile Phones',
  NOW()
);
-- Verify the insertion
SELECT
  COUNT(*) AS total_categories
FROM
  categories;
SELECT
  id,
  name,
  slug,
  level,
  display_order
FROM
  categories
ORDER BY
  level,
  display_order
LIMIT 10;
