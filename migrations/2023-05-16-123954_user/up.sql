-- Create the user table
CREATE TABLE users (
  id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  username VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(512) NOT NULL,
  full_name VARCHAR(255) NOT NULL,
  profile_image VARCHAR(255),
  session_token_id UUID,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create an index on the email column for efficient lookups
CREATE UNIQUE INDEX idx_users_email ON users(email);