-- Custom ENUM types for the application

CREATE TYPE entity_status AS ENUM ('active', 'inactive', 'pending', 'suspended');

CREATE TYPE account_level AS ENUM ('free', 'premium', 'enterprise');

CREATE TYPE account_status AS ENUM ('active', 'pending', 'suspended', 'banned', 'deactivated');

CREATE TYPE admin_role AS ENUM ('superadmin', 'admin', 'moderator');

CREATE TYPE token_type AS ENUM ('access', 'refresh', 'adminaccess', 'adminrefresh');

CREATE TYPE validation_type AS ENUM ('emailverification', 'passwordreset', 'twofactorauth', 'admininvite', 'accountactivation');
