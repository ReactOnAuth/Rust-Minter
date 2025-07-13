-- Quick fix for RLS policy error
-- Run this in your Supabase SQL Editor to allow anonymous inserts

-- Allow anonymous users to insert new addresses
CREATE POLICY "Allow anonymous insert for mint addresses" ON mint_addresses
    FOR INSERT WITH CHECK (true);

-- Optional: If you want to allow anonymous users to also update addresses
-- CREATE POLICY "Allow anonymous update for mint addresses" ON mint_addresses
--     FOR UPDATE USING (true) WITH CHECK (true);

-- Verify the policies are created
SELECT schemaname, tablename, policyname, permissive, roles, cmd, qual, with_check
FROM pg_policies 
WHERE tablename = 'mint_addresses'; 