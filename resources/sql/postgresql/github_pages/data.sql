INSERT INTO
    github_pages (owner, repository, base_url)
VALUES (
        'euv-dev',
        'euv',
        'https://euv-dev.github.io/euv/'
    )
ON CONFLICT (owner, repository) DO NOTHING;