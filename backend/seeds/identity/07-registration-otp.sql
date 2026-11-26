INSERT INTO identity_registration_otp (id, channel, destination, code_digest, attempts, expires_at, proof_digest, verified_until) VALUES
    ('seed-otp-pending', 'email', 'seed.pending@example.com', decode('5e5752f73e97e8ea5eb8308b68777c3f5a668060261298a2c125e22c9e0360c1', 'hex'), 0, now() + interval '15 minutes', NULL, NULL),
    ('seed-otp-locked', 'email', 'seed.locked@example.com', decode('deb4a652884a9eed1e7942fd5235815eb9896e9a994d4272d1b3f4746909707b', 'hex'), 5, now() + interval '15 minutes', NULL, NULL),
    ('seed-otp-expired', 'email', 'seed.expired@example.com', decode('f5c21e5ace99135858a9c854c36899cdab5e8b0906b263dd14ad0f6ba32564a2', 'hex'), 0, now() - interval '1 minute', NULL, NULL),
    ('seed-otp-verified', 'email', 'seed.verified@example.com', decode('c1b40e73de80de9aef0d61aa4297b68f48f02b11365d249e655bd07605f7340b', 'hex'), 0, now() + interval '15 minutes', decode('ad1154eebbcb2a5b8c2f9155b4f451c13bc9827f6102924c06a0e51f1d56990b', 'hex'), now() + interval '30 minutes')
ON CONFLICT DO NOTHING;
