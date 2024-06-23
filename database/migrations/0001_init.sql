CREATE TABLE Users (
    user_id         UUID DEFAULT gen_random_uuid() NOT NULL,
    name            VARCHAR(50) NOT NULL,
    email           VARCHAR(50) UNIQUE NOT NULL,
    password        VARCHAR(255) NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at      TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (user_id)
);
CREATE INDEX "user_emails" ON Users(email);

CREATE TABLE Organization (
    org_id          UUID DEFAULT gen_random_uuid() NOT NULL,
    name            VARCHAR(50) UNIQUE NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at      TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (org_id)
);
CREATE INDEX "org_name" ON Organization (name);

CREATE TYPE UserAccessStatus AS ENUM (
    'Admin',
    'Member',
    'Guest'
);

CREATE TABLE UserOrganization (
    user_id      UUID NOT NULL,
    org_id       UUID NOT NULL,
    access       UserAccessStatus NOT NULL,
    PRIMARY KEY (user_id, org_id),
    FOREIGN KEY (user_id) REFERENCES Users (user_id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (org_id) REFERENCES Organization (org_id) ON DELETE CASCADE ON UPDATE NO ACTION
);

CREATE TABLE Application (
    app_id          UUID DEFAULT gen_random_uuid() NOT NULL,
    org_id          UUID NOT NULL,
    name            VARCHAR(50) NOT NULL,
    description     VARCHAR(255) NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at      TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (app_id),
    FOREIGN KEY (org_id) REFERENCES Organization (org_id) ON DELETE CASCADE ON UPDATE NO ACTION
    -- ON DELETE CASCADE
);
CREATE INDEX "application_name" ON Application(name);

CREATE TABLE UserApplication (
    user_id      UUID NOT NULL,
    app_id       UUID NOT NULL,
    access       UserAccessStatus NOT NULL,
    PRIMARY KEY (user_id, app_id),
    FOREIGN KEY (user_id) REFERENCES Users (user_id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (app_id) REFERENCES Application (app_id) ON DELETE CASCADE ON UPDATE NO ACTION
);

CREATE TABLE ApplocationLogHeadder (
    headder_id      UUID DEFAULT gen_random_uuid() NOT NULL,
    app_id          UUID NOT NULL,
    headder_version INT NOT NULL,
    headder         VARCHAR(50)[] NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    PRIMARY KEY(headder_id),
    UNIQUE (app_id, headder_version),
    FOREIGN KEY (app_id) REFERENCES Application (app_id) ON DELETE CASCADE ON UPDATE NO ACTION
);

CREATE TYPE LOGLEVEL AS ENUM (
    'WARNING',
    'INFO,',
    'ERROR',
    'FATAL'
);

CREATE TABLE ApplocationLog (
    app_id          UUID NOT NULL,
    headder_id      UUID NOT NULL,
    logged_at       TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    log_level       LogLevel NOT NULL,
    log             VARCHAR(50)[] NOT NULL,
    FOREIGN KEY (app_id) REFERENCES Application (app_id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (headder_id) REFERENCES ApplocationLogHeadder (headder_id)
);

CREATE TYPE KeyAccessPermission AS ENUM (
    'readonly',
    'writeaccess'
);

CREATE TABLE ApplicationKeys (
    app_id          UUID NOT NULL,
    key_id          VARCHAR(128) NOT NULL,
    key_name        VARCHAR(50) NOT NULL,
    key_access      KeyAccessPermission NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    PRIMARY KEY (app_id, key_id),
    FOREIGN KEY (app_id) REFERENCES Application (app_id) ON DELETE CASCADE ON UPDATE NO ACTION
);
