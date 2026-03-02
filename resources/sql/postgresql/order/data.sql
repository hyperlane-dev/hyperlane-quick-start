INSERT INTO
    order_user (
        username,
        password_hash,
        role,
        status
    )
VALUES (
        'root',
        '4813494d137e1631bba301d5acab6e7bb7aa74ce1185d456565ef51d737677b2',
        1,
        1
    )
ON CONFLICT (username) DO NOTHING;