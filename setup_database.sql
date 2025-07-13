-- Create the mint_addresses table
CREATE TABLE mint_addresses (
    id SERIAL PRIMARY KEY,
    pub_key TEXT NOT NULL UNIQUE,
    private_key TEXT NOT NULL,
    suffix_type TEXT NOT NULL CHECK (suffix_type IN ('pump', 'bonk')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create an index on suffix_type for faster queries
CREATE INDEX idx_mint_addresses_suffix_type ON mint_addresses(suffix_type);

-- Create an index on created_at for faster queries
CREATE INDEX idx_mint_addresses_created_at ON mint_addresses(created_at);

-- Add a comment to the table
COMMENT ON TABLE mint_addresses IS 'Stores generated Solana mint addresses with pump and bonk suffixes';

-- Add comments to columns
COMMENT ON COLUMN mint_addresses.pub_key IS 'The public key of the generated Solana address';
COMMENT ON COLUMN mint_addresses.private_key IS 'The base64-encoded private key of the generated address';
COMMENT ON COLUMN mint_addresses.suffix_type IS 'The type of suffix: pump or bonk';
COMMENT ON COLUMN mint_addresses.created_at IS 'Timestamp when the address was generated';

-- Enable Row Level Security (RLS)
ALTER TABLE mint_addresses ENABLE ROW LEVEL SECURITY;

-- Create a policy that allows all operations for authenticated users
CREATE POLICY "Allow all operations for authenticated users" ON mint_addresses
    FOR ALL USING (auth.role() = 'authenticated');

-- Create a policy that allows read access for anonymous users
CREATE POLICY "Allow read access for anonymous users" ON mint_addresses
    FOR SELECT USING (true);

-- Create a policy that allows anonymous users to insert new addresses
CREATE POLICY "Allow anonymous insert for mint addresses" ON mint_addresses
    FOR INSERT WITH CHECK (true); 