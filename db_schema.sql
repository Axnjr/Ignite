CREATE TABLE public.accounts (
  id INT8 NOT NULL DEFAULT unique_rowid(),
  user_id UUID NOT NULL,
  provider_id VARCHAR(255) NOT NULL,
  provider_type VARCHAR(255) NOT NULL,
  provider_account_id VARCHAR(255) NOT NULL,
  refresh_token STRING NULL,
  access_token STRING NOT NULL,
  expires_at TIMESTAMPTZ NULL,
  token_type VARCHAR(255) NULL,
  scope STRING NULL,
  id_token STRING NULL,
  session_state STRING NULL,
  CONSTRAINT accounts_pkey PRIMARY KEY (id ASC),
  CONSTRAINT accounts_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id)
)

CREATE TABLE public.auth_sessions (
  id INT8 NOT NULL DEFAULT unique_rowid(),
  expires TIMESTAMPTZ NOT NULL,
  session_token STRING NOT NULL,
  user_id UUID NOT NULL,
  CONSTRAINT auth_sessions_pkey PRIMARY KEY (id ASC),
  CONSTRAINT auth_sessions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id)
)

CREATE TABLE public."User" (
  id VARCHAR(255) NOT NULL,
  name VARCHAR(255) NULL,
  email VARCHAR(255) NULL,
  image VARCHAR(255) NULL,
  emailverified VARCHAR(255) NULL,
  CONSTRAINT "User_pkey" PRIMARY KEY (id ASC),
  UNIQUE INDEX "User_email_key" (email ASC)
)

CREATE TABLE public.userdetails (
  plantype STRING NULL,
  apikey VARCHAR(255) NULL,
  expiryon DATE NULL DEFAULT current_date() + '30 days':::INTERVAL::INTERVAL DAY,
  hits INT8 NULL,
  email VARCHAR(255) NULL,
  rowid INT8 NOT VISIBLE NOT NULL DEFAULT unique_rowid(),
  CONSTRAINT userdetails_pkey PRIMARY KEY (rowid ASC)
)

CREATE TABLE public.userkeystatus (
  id INT8 NOT NULL DEFAULT unique_rowid(),
  apikey VARCHAR(255) NULL,
  status VARCHAR(255) NULL,
  plantype STRING NULL DEFAULT 'Hobby':::STRING,
  CONSTRAINT userkeystatus_pkey PRIMARY KEY (id ASC)
)

CREATE TABLE public.users (
  id UUID NOT NULL DEFAULT uuid_generate_v4(),
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  email_verified BOOL NULL DEFAULT false,
  image STRING NULL,
  created_at TIMESTAMP NULL DEFAULT now():::TIMESTAMP,
  updated_at TIMESTAMP NULL DEFAULT now():::TIMESTAMP,
  CONSTRAINT users_pkey PRIMARY KEY (id ASC),
  UNIQUE INDEX users_email_key (email ASC)
)

CREATE TABLE public.verification_tokens (
  identifier VARCHAR(255) NOT NULL,
  token STRING NOT NULL,
  expires TIMESTAMPTZ NOT NULL,
  CONSTRAINT verification_tokens_pkey PRIMARY KEY (identifier ASC)
)