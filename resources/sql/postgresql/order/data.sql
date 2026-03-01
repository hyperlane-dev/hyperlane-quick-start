INSERT INTO
    order_user (
        username,
        password_hash,
        nickname,
        role,
        status
    )
VALUES (
        'root',
        '4813494d137e1631bba301d5acab6e7bb7aa74ce1185d456565ef51d737677b2',
        'Administrator',
        'admin',
        'approved'
    ) ON CONFLICT (username) DO NOTHING;