-- Forward-only correction: preserve the already-published viewer_access migration checksum.
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM uac_role WHERE id='analytics-admin' AND role_status='active')
       OR NOT EXISTS (SELECT 1 FROM uac_acl WHERE id='analytics-security'
           AND acl_category='page' AND acl_parent_id='analytics-token' AND acl_status='active') THEN
        RAISE EXCEPTION 'Expected Analytics admin role and security page are missing or inactive';
    END IF;
END $$;
INSERT INTO uac_role_acl(role_id,acl_id,created_at)
VALUES('analytics-admin','analytics-security',now())
ON CONFLICT (role_id,acl_id) DO NOTHING;
