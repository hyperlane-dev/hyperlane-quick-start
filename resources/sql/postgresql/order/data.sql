INSERT INTO
    order_user (
        username,
        password_hash,
        role,
        status
    )
VALUES (
        'root',
        '63a9f0ea7bb98050796b649e85481845',
        1,
        1
    )
ON CONFLICT (username) DO NOTHING;