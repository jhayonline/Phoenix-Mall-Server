-- seed_all_category_specs.sql
-- This script adds specifications for ALL categories in your marketplace

DO $$
DECLARE
    -- Mobility level 2
    cars_id UUID := 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee';
    motorbikes_id UUID := 'ffffffff-ffff-ffff-ffff-ffffffffffff';
    bicycles_id UUID := '00000000-0000-0000-0000-000000000001';

    -- Real Estate level 2 & 3
    rentals_id UUID := '00000000-0000-0000-0000-000000000002';
    properties_sale_id UUID := '00000000-0000-0000-0000-000000000003';
    houses_rent_id UUID := '00000000-0000-0000-0000-000000000004';
    apartments_rent_id UUID := '00000000-0000-0000-0000-000000000005';
    hostels_id UUID := '00000000-0000-0000-0000-000000000006';
    offices_id UUID := '00000000-0000-0000-0000-000000000007';
    shops_rent_id UUID := '00000000-0000-0000-0000-000000000008';
    houses_sale_id UUID := '00000000-0000-0000-0000-000000000009';
    buildings_id UUID := '00000000-0000-0000-0000-000000000010';
    apartments_sale_id UUID := '00000000-0000-0000-0000-000000000011';
    shops_sale_id UUID := '00000000-0000-0000-0000-000000000012';

    -- Mobile Devices level 2
    smartphones_id UUID := '00000000-0000-0000-0000-000000000013';
    tablets_id UUID := '00000000-0000-0000-0000-000000000014';
    mobile_accessories_id UUID := '00000000-0000-0000-0000-000000000015';
    smartwatches_id UUID := '00000000-0000-0000-0000-000000000016';

    -- Computing & Electronics level 2
    laptops_id UUID := '00000000-0000-0000-0000-000000000017';
    desktops_id UUID := '00000000-0000-0000-0000-000000000018';
    monitors_id UUID := '00000000-0000-0000-0000-000000000019';
    computer_components_id UUID := '00000000-0000-0000-0000-000000000020';
    computer_accessories_id UUID := '00000000-0000-0000-0000-000000000021';
    system_units_id UUID := '00000000-0000-0000-0000-000000000022';
    televisions_id UUID := '00000000-0000-0000-0000-000000000023';
    audio_equipment_id UUID := '00000000-0000-0000-0000-000000000024';
    headphones_id UUID := '00000000-0000-0000-0000-000000000025';
    cameras_id UUID := '00000000-0000-0000-0000-000000000026';

    -- Games level 2
    video_games_id UUID := '00000000-0000-0000-0000-000000000027';
    consoles_id UUID := '00000000-0000-0000-0000-000000000028';
    board_games_id UUID := '00000000-0000-0000-0000-000000000029';

    -- Home & Living level 2 & 3
    furniture_id UUID := '00000000-0000-0000-0000-000000000030';
    lighting_id UUID := '00000000-0000-0000-0000-000000000031';
    appliances_id UUID := '00000000-0000-0000-0000-000000000032';
    cookware_id UUID := '00000000-0000-0000-0000-000000000033';
    kitchen_appliances_id UUID := '00000000-0000-0000-0000-000000000034';
    home_appliances_id UUID := '00000000-0000-0000-0000-000000000035';

    -- Fashion & Style level 2 & 3
    men_id UUID := '00000000-0000-0000-0000-000000000036';
    women_id UUID := '00000000-0000-0000-0000-000000000037';
    kids_id UUID := '00000000-0000-0000-0000-000000000038';
    babies_id UUID := '00000000-0000-0000-0000-000000000039';
    men_clothing_id UUID := '00000000-0000-0000-0000-000000000040';
    men_footwear_id UUID := '00000000-0000-0000-0000-000000000041';
    men_accessories_id UUID := '00000000-0000-0000-0000-000000000042';
    women_clothing_id UUID := '00000000-0000-0000-0000-000000000043';
    women_footwear_id UUID := '00000000-0000-0000-0000-000000000044';
    women_accessories_id UUID := '00000000-0000-0000-0000-000000000045';
    kids_clothing_id UUID := '00000000-0000-0000-0000-000000000046';
    kids_footwear_id UUID := '00000000-0000-0000-0000-000000000047';
    kids_accessories_id UUID := '00000000-0000-0000-0000-000000000048';
    babies_clothing_id UUID := '00000000-0000-0000-0000-000000000049';
    babies_footwear_id UUID := '00000000-0000-0000-0000-000000000050';
    babies_accessories_id UUID := '00000000-0000-0000-0000-000000000051';

    -- Jewelry level 2 & 3
    men_jewelry_id UUID := '00000000-0000-0000-0000-000000000052';
    women_jewelry_id UUID := '00000000-0000-0000-0000-000000000053';
    men_rings_id UUID := '00000000-0000-0000-0000-000000000054';
    men_watches_id UUID := '00000000-0000-0000-0000-000000000055';
    men_necklaces_id UUID := '00000000-0000-0000-0000-000000000056';
    men_bracelets_id UUID := '00000000-0000-0000-0000-000000000057';
    cufflinks_id UUID := '00000000-0000-0000-0000-000000000058';
    women_rings_id UUID := '00000000-0000-0000-0000-000000000059';
    women_watches_id UUID := '00000000-0000-0000-0000-000000000060';
    women_necklaces_id UUID := '00000000-0000-0000-0000-000000000061';
    beads_id UUID := '00000000-0000-0000-0000-000000000062';
    bracelets_bangles_id UUID := '00000000-0000-0000-0000-000000000063';
    anklets_id UUID := '00000000-0000-0000-0000-000000000064';

    -- Beauty & Personal Care level 2
    hair_oral_id UUID := '00000000-0000-0000-0000-000000000065';
    skincare_id UUID := '00000000-0000-0000-0000-000000000066';
    fragrances_id UUID := '00000000-0000-0000-0000-000000000067';
    makeup_id UUID := '00000000-0000-0000-0000-000000000068';
    intimate_wellness_id UUID := '00000000-0000-0000-0000-000000000069';
    beauty_accessories_id UUID := '00000000-0000-0000-0000-000000000070';

    -- Baby & Kids level 2
    baby_fashion_id UUID := '00000000-0000-0000-0000-000000000071';
    toys_id UUID := '00000000-0000-0000-0000-000000000072';
    baby_food_id UUID := '00000000-0000-0000-0000-000000000073';
    baby_accessories_id UUID := '00000000-0000-0000-0000-000000000074';
    playground_id UUID := '00000000-0000-0000-0000-000000000075';

    -- Services level 2
    home_services_id UUID := '00000000-0000-0000-0000-000000000076';
    tech_services_id UUID := '00000000-0000-0000-0000-000000000077';
    repair_services_id UUID := '00000000-0000-0000-0000-000000000078';
    other_services_id UUID := '00000000-0000-0000-0000-000000000079';

    -- Food & Agriculture level 2
    preserved_food_id UUID := '00000000-0000-0000-0000-000000000080';
    fresh_food_id UUID := '00000000-0000-0000-0000-000000000081';
    farm_produce_id UUID := '00000000-0000-0000-0000-000000000082';
    agri_supplies_id UUID := '00000000-0000-0000-0000-000000000083';

    -- Pets & Animals level 2
    pet_accessories_id UUID := '00000000-0000-0000-0000-000000000084';
    pet_food_id UUID := '00000000-0000-0000-0000-000000000085';
    cats_id UUID := '00000000-0000-0000-0000-000000000086';
    dogs_id UUID := '00000000-0000-0000-0000-000000000087';
    fish_id UUID := '00000000-0000-0000-0000-000000000088';
    birds_id UUID := '00000000-0000-0000-0000-000000000089';
    rabbits_id UUID := '00000000-0000-0000-0000-000000000090';
    other_animals_id UUID := '00000000-0000-0000-0000-000000000091';

BEGIN
    -- ==================== CARS (Already has specs, adding more) ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), cars_id, 'Make', 'select', true, '["Toyota","Honda","Nissan","Kia","Hyundai","Mercedes-Benz","BMW","Audi","Ford","Volkswagen","Mazda","Suzuki","Mitsubishi","Lexus","Jeep","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), cars_id, 'Model', 'text', true, NULL, NULL, 'e.g., Corolla, Civic, C-Class', NULL, 2, NOW()),
    (gen_random_uuid(), cars_id, 'Year', 'select', true, '["2024","2023","2022","2021","2020","2019","2018","2017","2016","2015","2014","2013","2012","2011","2010","2009","2008","2007","2006","2005"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), cars_id, 'Mileage (km)', 'number', true, NULL, '{"min": 0, "max": 500000}', 'Enter kilometers', NULL, 4, NOW()),
    (gen_random_uuid(), cars_id, 'Transmission', 'select', true, '["Manual","Automatic","CVT","Semi-Automatic"]', NULL, NULL, NULL, 5, NOW()),
    (gen_random_uuid(), cars_id, 'Fuel Type', 'select', true, '["Petrol","Diesel","Electric","Hybrid","Plug-in Hybrid"]', NULL, NULL, NULL, 6, NOW()),
    (gen_random_uuid(), cars_id, 'Body Type', 'select', true, '["Sedan","SUV","Hatchback","Coupe","Convertible","Truck","Van","Wagon"]', NULL, NULL, NULL, 7, NOW()),
    (gen_random_uuid(), cars_id, 'Condition', 'select', true, '["Brand New","Used - Excellent","Used - Good","Used - Fair"]', NULL, NULL, NULL, 8, NOW()),
    (gen_random_uuid(), cars_id, 'Color', 'select', true, '["Black","White","Silver","Gray","Blue","Red","Green","Brown","Yellow","Orange","Purple"]', NULL, NULL, NULL, 9, NOW()),
    (gen_random_uuid(), cars_id, 'Engine Size (L)', 'text', false, NULL, NULL, 'e.g., 1.8L, 2.0L, 3.5L V6', NULL, 10, NOW()),
    (gen_random_uuid(), cars_id, 'Drive Type', 'select', false, '["FWD","RWD","AWD","4WD"]', NULL, NULL, NULL, 11, NOW());

    -- ==================== MOTORBIKES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), motorbikes_id, 'Make', 'select', true, '["Honda","Yamaha","Suzuki","Kawasaki","TVS","Bajaj","Ducati","Harley-Davidson","BMW","KTM","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), motorbikes_id, 'Model', 'text', true, NULL, NULL, 'e.g., CBR, R15, Ninja', NULL, 2, NOW()),
    (gen_random_uuid(), motorbikes_id, 'Year', 'select', true, '["2024","2023","2022","2021","2020","2019","2018","2017","2016","2015"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), motorbikes_id, 'Engine Capacity (cc)', 'select', true, '["50cc","125cc","150cc","200cc","250cc","300cc","400cc","600cc","1000cc","Above 1000cc"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), motorbikes_id, 'Mileage (km)', 'number', true, NULL, '{"min": 0, "max": 200000}', 'Enter kilometers', NULL, 5, NOW()),
    (gen_random_uuid(), motorbikes_id, 'Condition', 'select', true, '["Brand New","Used - Excellent","Used - Good","Used - Fair"]', NULL, NULL, NULL, 6, NOW()),
    (gen_random_uuid(), motorbikes_id, 'Color', 'select', true, '["Black","White","Red","Blue","Green","Yellow","Orange","Gray","Other"]', NULL, NULL, NULL, 7, NOW());

    -- ==================== BICYCLES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), bicycles_id, 'Type', 'select', true, '["Mountain Bike","Road Bike","Hybrid Bike","Electric Bike","BMX","Cruiser","Folding Bike","Kids Bike"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), bicycles_id, 'Brand', 'select', true, '["Trek","Giant","Specialized","Cannondale","Scott","Merida","GT","Schwinn","Raleigh","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), bicycles_id, 'Frame Size', 'select', true, '["XS","S","M","L","XL","XXL"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), bicycles_id, 'Wheel Size', 'select', true, '["12\"","16\"","20\"","24\"","26\"","27.5\"","29\""]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), bicycles_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair","Needs Repair"]', NULL, NULL, NULL, 5, NOW()),
    (gen_random_uuid(), bicycles_id, 'Color', 'select', false, '["Black","White","Red","Blue","Green","Yellow","Orange","Purple"]', NULL, NULL, NULL, 6, NOW());

    -- ==================== REAL ESTATE - RENTALS (General) ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), rentals_id, 'Property Type', 'select', true, '["House","Apartment","Hostel","Office","Shop","Studio","Townhouse","Villa"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), rentals_id, 'Bedrooms', 'select', true, '["Studio","1 Bedroom","2 Bedrooms","3 Bedrooms","4 Bedrooms","5+ Bedrooms"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), rentals_id, 'Bathrooms', 'select', true, '["1","2","3","4","5+"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), rentals_id, 'Furnishing', 'select', true, '["Fully Furnished","Semi-Furnished","Unfurnished"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), rentals_id, 'Rent Period', 'select', true, '["Monthly","Quarterly","Yearly","Long-term"]', NULL, NULL, NULL, 5, NOW()),
    (gen_random_uuid(), rentals_id, 'Square Footage', 'number', false, NULL, '{"min": 0, "max": 10000}', 'Enter area in sq ft', NULL, 6, NOW()),
    (gen_random_uuid(), rentals_id, 'Parking', 'boolean', false, NULL, NULL, NULL, 'Has parking space', 7, NOW()),
    (gen_random_uuid(), rentals_id, 'Pets Allowed', 'boolean', false, NULL, NULL, NULL, 'Pets allowed', 8, NOW());

    -- ==================== REAL ESTATE - PROPERTIES FOR SALE ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), properties_sale_id, 'Property Type', 'select', true, '["House","Building","Apartment","Shop","Land","Townhouse","Villa","Warehouse"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), properties_sale_id, 'Bedrooms', 'select', false, '["Studio","1 Bedroom","2 Bedrooms","3 Bedrooms","4 Bedrooms","5+ Bedrooms"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), properties_sale_id, 'Bathrooms', 'select', false, '["1","2","3","4","5+"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), properties_sale_id, 'Land Size', 'text', false, NULL, NULL, 'e.g., 0.5 acres, 1000 sq meters', NULL, 4, NOW()),
    (gen_random_uuid(), properties_sale_id, 'Title Status', 'select', true, '["Freehold","Leasehold","Government"]', NULL, NULL, NULL, 5, NOW()),
    (gen_random_uuid(), properties_sale_id, 'Year Built', 'number', false, NULL, '{"min": 1900, "max": 2024}', 'Enter year', NULL, 6, NOW());

    -- ==================== HOUSES FOR RENT ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), houses_rent_id, 'House Type', 'select', true, '["Detached","Semi-Detached","Townhouse","Mansion","Bungalow"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), houses_rent_id, 'Bedrooms', 'select', true, '["1","2","3","4","5+"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), houses_rent_id, 'Bathrooms', 'select', true, '["1","2","3","4","5+"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), houses_rent_id, 'Stories', 'select', true, '["1","2","3","4+"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), houses_rent_id, 'Furnishing', 'select', true, '["Fully Furnished","Semi-Furnished","Unfurnished"]', NULL, NULL, NULL, 5, NOW());

    -- ==================== APARTMENTS FOR RENT ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), apartments_rent_id, 'Apartment Type', 'select', true, '["Studio","1 Bedroom","2 Bedroom","3 Bedroom","Penthouse","Duplex"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), apartments_rent_id, 'Floor Level', 'select', true, '["Ground Floor","1st Floor","2nd Floor","3rd Floor","4th Floor","5th+ Floor","Top Floor"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), apartments_rent_id, 'Elevator', 'boolean', false, NULL, NULL, NULL, 'Has elevator', 3, NOW()),
    (gen_random_uuid(), apartments_rent_id, 'Security', 'select', false, '["24/7 Security","CCTV","Security Guard","None"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== HOSTELS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), hostels_id, 'Room Type', 'select', true, '["Single Room","Double Room","4-Bed","6-Bed","8-Bed","10-Bed"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), hostels_id, 'Meal Plan', 'select', true, '["Self Catering","Breakfast Only","Half Board","Full Board"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), hostels_id, 'Gender', 'select', true, '["Male Only","Female Only","Mixed"]', NULL, NULL, NULL, 3, NOW());

    -- ==================== OFFICES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), offices_id, 'Office Type', 'select', true, '["Private Office","Shared Office","Co-working Space","Executive Suite"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), offices_id, 'Workstations', 'number', true, NULL, '{"min": 1, "max": 500}', 'Number of workstations', NULL, 2, NOW()),
    (gen_random_uuid(), offices_id, 'Meeting Rooms', 'number', false, NULL, '{"min": 0, "max": 20}', 'Number of meeting rooms', NULL, 3, NOW()),
    (gen_random_uuid(), offices_id, 'Parking Spots', 'number', false, NULL, '{"min": 0, "max": 100}', NULL, NULL, 4, NOW());

    -- ==================== SHOPS FOR RENT ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), shops_rent_id, 'Shop Type', 'select', true, '["Retail","Showroom","Kiosk","Container Shop","Mall Space"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), shops_rent_id, 'Frontage', 'text', false, NULL, NULL, 'e.g., 10 meters, 15 ft', 'Street frontage width', 2, NOW());

    -- ==================== TABLETS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), tablets_id, 'Brand', 'select', true, '["Apple","Samsung","Huawei","Lenovo","Microsoft","Amazon","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), tablets_id, 'Model', 'text', true, NULL, NULL, 'e.g., iPad Pro, Galaxy Tab S9', NULL, 2, NOW()),
    (gen_random_uuid(), tablets_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), tablets_id, 'Screen Size', 'select', true, '["7\"","8\"","10\"","11\"","12.9\"","13\""]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), tablets_id, 'Storage', 'select', true, '["32GB","64GB","128GB","256GB","512GB","1TB"]', NULL, NULL, NULL, 5, NOW()),
    (gen_random_uuid(), tablets_id, 'Cellular', 'boolean', false, NULL, NULL, NULL, 'Supports SIM card', 6, NOW()),
    (gen_random_uuid(), tablets_id, 'Color', 'select', false, '["Space Gray","Silver","Gold","Rose Gold","Black","White"]', NULL, NULL, NULL, 7, NOW());

    -- ==================== MOBILE ACCESSORIES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), mobile_accessories_id, 'Accessory Type', 'select', true, '["Phone Case","Screen Protector","Charger","Power Bank","Cable","Phone Stand","Pop Socket","Car Mount","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), mobile_accessories_id, 'Compatible Brand', 'select', true, '["Apple","Samsung","Huawei","Xiaomi","Tecno","Infinix","Universal"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), mobile_accessories_id, 'Compatible Model', 'text', false, NULL, NULL, 'e.g., iPhone 14 Pro Max', NULL, 3, NOW()),
    (gen_random_uuid(), mobile_accessories_id, 'Color', 'select', false, '["Black","White","Clear","Blue","Red","Green","Pink","Purple"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== SMARTWATCHES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), smartwatches_id, 'Brand', 'select', true, '["Apple","Samsung","Garmin","Fitbit","Xiaomi","Amazfit","Huawei","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), smartwatches_id, 'Model', 'text', true, NULL, NULL, 'e.g., Apple Watch Series 9', NULL, 2, NOW()),
    (gen_random_uuid(), smartwatches_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), smartwatches_id, 'Case Size', 'select', true, '["40mm","41mm","44mm","45mm","49mm"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), smartwatches_id, 'Cellular', 'boolean', false, NULL, NULL, NULL, 'Has cellular connectivity', 5, NOW()),
    (gen_random_uuid(), smartwatches_id, 'Band Color', 'select', false, '["Black","White","Blue","Red","Green","Starlight"]', NULL, NULL, NULL, 6, NOW());

    -- ==================== DESKTOP COMPUTERS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), desktops_id, 'Brand', 'select', true, '["Dell","HP","Lenovo","Apple","ASUS","Acer","Custom Built","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), desktops_id, 'Model', 'text', true, NULL, NULL, 'e.g., Optiplex, Pavilion, Mac', NULL, 2, NOW()),
    (gen_random_uuid(), desktops_id, 'Processor', 'select', true, '["Intel Core i3","Intel Core i5","Intel Core i7","Intel Core i9","AMD Ryzen 5","AMD Ryzen 7","AMD Ryzen 9","Apple M1","Apple M2"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), desktops_id, 'RAM', 'select', true, '["4GB","8GB","16GB","32GB","64GB","128GB"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), desktops_id, 'Storage', 'select', true, '["128GB","256GB","512GB","1TB","2TB","4TB"]', NULL, NULL, NULL, 5, NOW()),
    (gen_random_uuid(), desktops_id, 'Storage Type', 'select', true, '["SSD","HDD","SSD + HDD"]', NULL, NULL, NULL, 6, NOW()),
    (gen_random_uuid(), desktops_id, 'Graphics Card', 'text', false, NULL, NULL, 'e.g., NVIDIA RTX 3060, Integrated', NULL, 7, NOW()),
    (gen_random_uuid(), desktops_id, 'Operating System', 'select', true, '["Windows 10","Windows 11","macOS","Linux","No OS"]', NULL, NULL, NULL, 8, NOW()),
    (gen_random_uuid(), desktops_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 9, NOW());

    -- ==================== MONITORS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), monitors_id, 'Brand', 'select', true, '["Dell","HP","LG","Samsung","ASUS","Acer","BenQ","MSI","ViewSonic","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), monitors_id, 'Screen Size', 'select', true, '["19\"","21\"","22\"","24\"","27\"","32\"","34\"","43\"","49\""]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), monitors_id, 'Resolution', 'select', true, '["HD (1366x768)","Full HD (1920x1080)","2K (2560x1440)","4K (3840x2160)","Ultrawide"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), monitors_id, 'Refresh Rate', 'select', true, '["60Hz","75Hz","120Hz","144Hz","165Hz","240Hz"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), monitors_id, 'Response Time', 'select', false, '["1ms","2ms","4ms","5ms","8ms"]', NULL, NULL, NULL, 5, NOW()),
    (gen_random_uuid(), monitors_id, 'Panel Type', 'select', true, '["IPS","TN","VA","OLED","Mini LED"]', NULL, NULL, NULL, 6, NOW()),
    (gen_random_uuid(), monitors_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 7, NOW());

    -- ==================== COMPUTER COMPONENTS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), computer_components_id, 'Component Type', 'select', true, '["CPU","GPU","Motherboard","RAM","SSD","HDD","Power Supply","CPU Cooler","Case Fan","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), computer_components_id, 'Brand', 'select', true, '["Intel","AMD","NVIDIA","Corsair","Kingston","Samsung","WD","Seagate","ASUS","MSI","Gigabyte","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), computer_components_id, 'Model/Specs', 'text', true, NULL, NULL, 'e.g., i7-13700K, RTX 4070, 16GB DDR5', NULL, 3, NOW()),
    (gen_random_uuid(), computer_components_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== COMPUTER ACCESSORIES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), computer_accessories_id, 'Accessory Type', 'select', true, '["Keyboard","Mouse","Headset","Speakers","Webcam","Mouse Pad","USB Hub","Docking Station","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), computer_accessories_id, 'Connection', 'select', true, '["Wired","Wireless","Bluetooth","USB","USB-C"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), computer_accessories_id, 'Brand', 'select', true, '["Logitech","Razer","Corsair","HyperX","SteelSeries","Microsoft","Dell","HP","Other"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), computer_accessories_id, 'Color', 'select', false, '["Black","White","RGB","Gray","Silver","Other"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== TELEVISIONS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), televisions_id, 'Brand', 'select', true, '["Samsung","LG","Sony","TCL","Hisense","Panasonic","Sharp","Xiaomi","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), televisions_id, 'Screen Size', 'select', true, '["32\"","40\"","43\"","50\"","55\"","65\"","75\"","85\"","98\""]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), televisions_id, 'Resolution', 'select', true, '["HD Ready","Full HD","4K Ultra HD","8K"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), televisions_id, 'Display Type', 'select', true, '["LED","OLED","QLED","Mini LED","Plasma"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), televisions_id, 'Smart TV', 'boolean', true, NULL, NULL, NULL, 'Has smart features', 5, NOW()),
    (gen_random_uuid(), televisions_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 6, NOW());

    -- ==================== AUDIO EQUIPMENT ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), audio_equipment_id, 'Equipment Type', 'select', true, '["Speaker","Soundbar","Subwoofer","Amplifier","Receiver","Mixer","Turntable","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), audio_equipment_id, 'Brand', 'select', true, '["JBL","Sony","Bose","Samsung","LG","Yamaha","Pioneer","Polk","Klipsch","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), audio_equipment_id, 'Power (Watts)', 'number', false, NULL, '{"min": 0, "max": 5000}', 'Enter wattage', NULL, 3, NOW()),
    (gen_random_uuid(), audio_equipment_id, 'Bluetooth', 'boolean', false, NULL, NULL, NULL, 'Has Bluetooth', 4, NOW()),
    (gen_random_uuid(), audio_equipment_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 5, NOW());

    -- ==================== HEADPHONES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), headphones_id, 'Type', 'select', true, '["Over-Ear","On-Ear","In-Ear","True Wireless","Earbuds","Gaming Headset"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), headphones_id, 'Connection', 'select', true, '["Wired","Wireless","Bluetooth","USB","3.5mm"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), headphones_id, 'Noise Cancelling', 'boolean', false, NULL, NULL, NULL, 'Has noise cancellation', 3, NOW()),
    (gen_random_uuid(), headphones_id, 'Brand', 'select', true, '["Sony","Bose","Apple","Samsung","JBL","Beats","Sennheiser","Anker","Other"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), headphones_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 5, NOW());

    -- ==================== CAMERAS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), cameras_id, 'Camera Type', 'select', true, '["DSLR","Mirrorless","Point & Shoot","Action Camera","Instant Camera","Film Camera","Security Camera"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), cameras_id, 'Brand', 'select', true, '["Canon","Nikon","Sony","Fujifilm","Panasonic","GoPro","DJI","Leica","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), cameras_id, 'Megapixels', 'number', true, NULL, '{"min": 0, "max": 100}', 'Enter MP', NULL, 3, NOW()),
    (gen_random_uuid(), cameras_id, 'Lens Included', 'boolean', true, NULL, NULL, NULL, 'Includes lens', 4, NOW()),
    (gen_random_uuid(), cameras_id, 'Video Recording', 'select', false, '["1080p","4K","8K","None"]', NULL, NULL, NULL, 5, NOW()),
    (gen_random_uuid(), cameras_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 6, NOW());

    -- ==================== VIDEO GAMES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), video_games_id, 'Platform', 'select', true, '["PlayStation","Xbox","Nintendo Switch","PC","Mobile"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), video_games_id, 'Genre', 'select', true, '["Action","Adventure","RPG","Sports","Racing","Fighting","Horror","Strategy","Simulation","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), video_games_id, 'Format', 'select', true, '["Physical Disc","Digital Code"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), video_games_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== CONSOLES & CONTROLLERS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), consoles_id, 'Console Type', 'select', true, '["PlayStation 5","PlayStation 4","Xbox Series X/S","Xbox One","Nintendo Switch","Nintendo 3DS","PSP","Retro Console"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), consoles_id, 'Item Type', 'select', true, '["Console Only","Console + Controller","Full Bundle","Controller Only","Accessory"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), consoles_id, 'Storage', 'select', true, '["500GB","1TB","2TB","Expandable"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), consoles_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== FURNITURE ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), furniture_id, 'Furniture Type', 'select', true, '["Sofa","Bed","Dining Table","Chair","Wardrobe","Desk","Bookshelf","Nightstand","Dresser","Cabinet","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), furniture_id, 'Material', 'select', true, '["Wood","Metal","Leather","Fabric","Glass","Plastic","Rattan","Bamboo","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), furniture_id, 'Color', 'select', false, '["Black","White","Brown","Gray","Beige","Blue","Red","Green","Other"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), furniture_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), furniture_id, 'Assembly Required', 'boolean', false, NULL, NULL, NULL, 'Requires assembly', 5, NOW());

    -- ==================== LIGHTING ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), lighting_id, 'Lighting Type', 'select', true, '["Ceiling Light","Floor Lamp","Table Lamp","Wall Sconce","Chandelier","Pendant Light","Outdoor Light","String Lights","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), lighting_id, 'Bulb Type', 'select', true, '["LED","Incandescent","Fluorescent","Halogen","Smart Bulb"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), lighting_id, 'Wattage', 'number', false, NULL, '{"min": 0, "max": 200}', 'Enter wattage', NULL, 3, NOW()),
    (gen_random_uuid(), lighting_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== KITCHEN APPLIANCES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), kitchen_appliances_id, 'Appliance Type', 'select', true, '["Refrigerator","Microwave","Oven","Dishwasher","Blender","Air Fryer","Rice Cooker","Coffee Maker","Toaster","Food Processor","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), kitchen_appliances_id, 'Brand', 'select', true, '["Samsung","LG","Whirlpool","Kenmore","KitchenAid","GE","Bosch","Panasonic","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), kitchen_appliances_id, 'Energy Rating', 'select', false, '["A+++","A++","A+","A","B","C"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), kitchen_appliances_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== HOME APPLIANCES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), home_appliances_id, 'Appliance Type', 'select', true, '["Washing Machine","Dryer","Vacuum Cleaner","Air Conditioner","Heater","Fan","Humidifier","Air Purifier","Iron","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), home_appliances_id, 'Brand', 'select', true, '["Samsung","LG","Whirlpool","Dyson","Shark","Miele","Bosch","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), home_appliances_id, 'Energy Rating', 'select', false, '["A+++","A++","A+","A","B","C"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), home_appliances_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== MEN CLOTHING ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), men_clothing_id, 'Clothing Type', 'select', true, '["T-Shirt","Shirt","Pants","Jeans","Shorts","Jacket","Hoodie","Sweater","Blazer","Suit","Underwear","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), men_clothing_id, 'Size', 'select', true, '["XS","S","M","L","XL","XXL","3XL","4XL"]', NULL, NULL, 'Men''s sizes', 2, NOW()),
    (gen_random_uuid(), men_clothing_id, 'Color', 'select', true, '["Black","White","Blue","Red","Green","Gray","Brown","Navy","Beige","Other"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), men_clothing_id, 'Brand', 'text', false, NULL, NULL, 'Brand name', NULL, 4, NOW()),
    (gen_random_uuid(), men_clothing_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 5, NOW());

    -- ==================== MEN FOOTWEAR ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), men_footwear_id, 'Footwear Type', 'select', true, '["Sneakers","Boots","Sandals","Loafers","Formal Shoes","Running Shoes","Slides","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), men_footwear_id, 'Size (EU)', 'select', true, '["39","40","41","42","43","44","45","46","47","48","49","50"]', NULL, NULL, 'European sizes', 2, NOW()),
    (gen_random_uuid(), men_footwear_id, 'Color', 'select', true, '["Black","White","Brown","Gray","Blue","Red","Other"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), men_footwear_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== WOMEN CLOTHING ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), women_clothing_id, 'Clothing Type', 'select', true, '["Dress","Top","Blouse","Shirt","Pants","Jeans","Skirt","Jacket","Cardigan","Sweater","Jumpsuit","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), women_clothing_id, 'Size', 'select', true, '["XS","S","M","L","XL","XXL","Plus Size"]', NULL, NULL, 'Women''s sizes', 2, NOW()),
    (gen_random_uuid(), women_clothing_id, 'Color', 'select', true, '["Black","White","Red","Blue","Green","Pink","Purple","Yellow","Brown","Other"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), women_clothing_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== WOMEN FOOTWEAR ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), women_footwear_id, 'Footwear Type', 'select', true, '["Heels","Flats","Sneakers","Boots","Sandals","Wedges","Loafers","Running Shoes","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), women_footwear_id, 'Size (EU)', 'select', true, '["35","36","37","38","39","40","41","42","43"]', NULL, NULL, 'European sizes', 2, NOW()),
    (gen_random_uuid(), women_footwear_id, 'Heel Height', 'select', false, '["Flat","Low (1-2\")","Medium (2-3\")","High (3-4\")","Very High (4+\")"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), women_footwear_id, 'Color', 'select', true, '["Black","White","Nude","Red","Blue","Pink","Gold","Silver","Other"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== MEN JEWELRY - RINGS & EARRINGS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), men_rings_id, 'Jewelry Type', 'select', true, '["Ring","Earring","Set"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), men_rings_id, 'Metal', 'select', true, '["Gold","Silver","Platinum","Titanium","Stainless Steel","Tungsten","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), men_rings_id, 'Gold Karat', 'select', false, '["10K","14K","18K","22K","24K"]', NULL, NULL, 'If gold', 3, NOW()),
    (gen_random_uuid(), men_rings_id, 'Ring Size (US)', 'select', true, '["6","7","8","9","10","11","12","13","14"]', NULL, NULL, 'US ring size', 4, NOW());

    -- ==================== MEN WATCHES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), men_watches_id, 'Movement', 'select', true, '["Automatic","Quartz","Mechanical","Digital","Smartwatch"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), men_watches_id, 'Case Material', 'select', true, '["Stainless Steel","Gold","Silver","Titanium","Ceramic","Plastic","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), men_watches_id, 'Band Material', 'select', true, '["Leather","Metal","Silicone","Nylon","Rubber","Other"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), men_watches_id, 'Water Resistance', 'select', true, '["30m","50m","100m","200m","300m+"]', NULL, NULL, NULL, 4, NOW()),
    (gen_random_uuid(), men_watches_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 5, NOW());

    -- ==================== WOMEN RINGS & EARRINGS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), women_rings_id, 'Jewelry Type', 'select', true, '["Ring","Earring","Set","Stud","Hoop","Dangle"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), women_rings_id, 'Metal', 'select', true, '["Gold","Silver","Platinum","Rose Gold","White Gold","Other"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), women_rings_id, 'Gold Karat', 'select', false, '["10K","14K","18K","22K","24K"]', NULL, NULL, 'If gold', 3, NOW()),
    (gen_random_uuid(), women_rings_id, 'Gemstone', 'select', false, '["Diamond","Ruby","Sapphire","Emerald","Opal","Cubic Zirconia","None"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== WOMEN NECKLACES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), women_necklaces_id, 'Necklace Type', 'select', true, '["Pendant","Chain","Choker","Locket","Statement","Pearl"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), women_necklaces_id, 'Length (inches)', 'select', true, '["16\"","18\"","20\"","22\"","24\"","30\""]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), women_necklaces_id, 'Metal', 'select', true, '["Gold","Silver","Rose Gold","Platinum","Other"]', NULL, NULL, NULL, 3, NOW());

    -- ==================== SKINCARE ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), skincare_id, 'Product Type', 'select', true, '["Moisturizer","Serum","Cleanser","Toner","Exfoliator","Mask","Sunscreen","Eye Cream","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), skincare_id, 'Skin Type', 'select', true, '["Normal","Dry","Oily","Combination","Sensitive","All"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), skincare_id, 'Brand', 'text', false, NULL, NULL, 'Brand name', NULL, 3, NOW()),
    (gen_random_uuid(), skincare_id, 'Size (ml)', 'number', true, NULL, '{"min": 0, "max": 500}', 'Volume in ml', NULL, 4, NOW());

    -- ==================== HAIR & ORAL CARE ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), hair_oral_id, 'Product Type', 'select', true, '["Shampoo","Conditioner","Hair Oil","Hair Mask","Hair Spray","Hair Dye","Toothpaste","Mouthwash","Toothbrush","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), hair_oral_id, 'Hair Type', 'select', false, '["Straight","Wavy","Curly","Coily","All","Damaged","Colored"]', NULL, NULL, 'For hair products', 2, NOW()),
    (gen_random_uuid(), hair_oral_id, 'Size (ml)', 'number', true, NULL, '{"min": 0, "max": 1000}', 'Volume in ml', NULL, 3, NOW());

    -- ==================== MAKEUP ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), makeup_id, 'Product Type', 'select', true, '["Foundation","Concealer","Powder","Blush","Highlighter","Lipstick","Lip Gloss","Mascara","Eyeliner","Eyeshadow","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), makeup_id, 'Shade', 'text', true, NULL, NULL, 'e.g., Warm Beige, Ruby Red', 'Color/shade name', 2, NOW()),
    (gen_random_uuid(), makeup_id, 'Brand', 'text', false, NULL, NULL, 'Brand name', NULL, 3, NOW()),
    (gen_random_uuid(), makeup_id, 'Finish', 'select', false, '["Matte","Glossy","Satin","Shimmer","Metallic","Natural"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== FRAGRANCES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), fragrances_id, 'Fragrance Type', 'select', true, '["Perfume","Eau de Parfum","Eau de Toilette","Cologne","Body Spray","Oil"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), fragrances_id, 'Gender', 'select', true, '["Men","Women","Unisex"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), fragrances_id, 'Size (ml)', 'number', true, NULL, '{"min": 0, "max": 200}', 'Volume in ml', NULL, 3, NOW()),
    (gen_random_uuid(), fragrances_id, 'Brand', 'text', true, NULL, NULL, 'Brand name', NULL, 4, NOW());

    -- ==================== BABY FOOD & NUTRITION ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), baby_food_id, 'Product Type', 'select', true, '["Formula","Baby Cereal","Baby Snacks","Baby Juice","Puree","Teething Biscuits"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), baby_food_id, 'Age Range', 'select', true, '["0-6 months","6-12 months","12-18 months","18-24 months","2+ years"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), baby_food_id, 'Brand', 'text', true, NULL, NULL, 'Brand name', NULL, 3, NOW());

    -- ==================== TOYS ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), toys_id, 'Toy Type', 'select', true, '["Action Figure","Doll","Plush Toy","Building Blocks","Educational Toy","Remote Control","Board Game","Puzzle","Outdoor Toy","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), toys_id, 'Age Range', 'select', true, '["0-2 years","3-5 years","6-8 years","9-12 years","12+ years"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), toys_id, 'Brand', 'text', false, NULL, NULL, 'Brand name', NULL, 3, NOW()),
    (gen_random_uuid(), toys_id, 'Condition', 'select', true, '["Brand New","Like New","Used - Good","Used - Fair"]', NULL, NULL, NULL, 4, NOW());

    -- ==================== BABY ACCESSORIES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), baby_accessories_id, 'Accessory Type', 'select', true, '["Diapers","Wipes","Bottles","Pacifiers","Baby Carrier","Stroller","High Chair","Baby Monitor","Nursing Pillow","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), baby_accessories_id, 'Age Range', 'select', true, '["Newborn","0-3 months","3-6 months","6-12 months","12+ months"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), baby_accessories_id, 'Brand', 'text', false, NULL, NULL, 'Brand name', NULL, 3, NOW());

    -- ==================== HOME SERVICES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), home_services_id, 'Service Type', 'select', true, '["Cleaning","Plumbing","Electrical","Painting","Moving","Lawn Care","Pest Control","Construction","Renovation","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), home_services_id, 'Service Duration', 'select', true, '["One-time","Hourly","Daily","Weekly","Monthly","Project-based"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), home_services_id, 'Experience (years)', 'number', false, NULL, '{"min": 0, "max": 50}', 'Years of experience', NULL, 3, NOW());

    -- ==================== TECH SERVICES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), tech_services_id, 'Service Type', 'select', true, '["Web Development","Mobile App Development","IT Support","Data Recovery","Network Setup","Software Installation","Computer Repair","Laptop Repair","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), tech_services_id, 'Service Type', 'select', false, '["Remote","On-site","Both"]', NULL, NULL, 'Service delivery method', 2, NOW()),
    (gen_random_uuid(), tech_services_id, 'Response Time', 'select', false, '["Within 24h","Within 48h","Within 1 week","Flexible"]', NULL, NULL, NULL, 3, NOW());

    -- ==================== REPAIR & MAINTENANCE ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), repair_services_id, 'Repair Type', 'select', true, '["Phone Repair","Computer Repair","Appliance Repair","TV Repair","AC Repair","Generator Repair","Vehicle Repair","Shoe Repair","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), repair_services_id, 'Warranty', 'boolean', false, NULL, NULL, NULL, 'Offers warranty on repairs', 2, NOW()),
    (gen_random_uuid(), repair_services_id, 'Pick-up Available', 'boolean', false, NULL, NULL, NULL, 'Will pick up item', 3, NOW());

    -- ==================== FRESH FOOD ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), fresh_food_id, 'Food Type', 'select', true, '["Fruits","Vegetables","Meat","Seafood","Dairy","Eggs","Bakery","Herbs","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), fresh_food_id, 'Organic', 'boolean', false, NULL, NULL, NULL, 'Organic certified', 2, NOW()),
    (gen_random_uuid(), fresh_food_id, 'Expiration Days', 'number', true, NULL, '{"min": 1, "max": 30}', 'Days until expiry', 'Approximate shelf life', 3, NOW());

    -- ==================== PET ACCESSORIES ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), pet_accessories_id, 'Accessory Type', 'select', true, '["Collar","Leash","Harness","Bed","Carrier","Toy","Bowl","Grooming Tool","Clothing","Other"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), pet_accessories_id, 'Pet Type', 'select', true, '["Dog","Cat","Bird","Fish","Rabbit","Small Animal","Multiple"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), pet_accessories_id, 'Size', 'select', true, '["XS","S","M","L","XL","One Size"]', NULL, NULL, NULL, 3, NOW());

    -- ==================== PET FOOD ====================
    INSERT INTO category_specs (id, category_id, spec_name, spec_type, is_required, preset_options, validation_rules, input_placeholder, helper_text, sort_order, created_at) VALUES
    (gen_random_uuid(), pet_food_id, 'Food Type', 'select', true, '["Dry Kibble","Wet Food","Raw Food","Treats","Supplements","Prescription Diet"]', NULL, NULL, NULL, 1, NOW()),
    (gen_random_uuid(), pet_food_id, 'Pet Type', 'select', true, '["Dog","Cat","Bird","Fish","Rabbit","Multiple"]', NULL, NULL, NULL, 2, NOW()),
    (gen_random_uuid(), pet_food_id, 'Life Stage', 'select', true, '["Puppy/Kitten","Adult","Senior","All Stages"]', NULL, NULL, NULL, 3, NOW()),
    (gen_random_uuid(), pet_food_id, 'Weight (kg)', 'number', true, NULL, '{"min": 0, "max": 50}', 'Package weight in kg', NULL, 4, NOW());

    RAISE NOTICE 'All category specifications seeded successfully!';
END $$;