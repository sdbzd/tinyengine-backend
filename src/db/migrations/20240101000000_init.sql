CREATE TABLE IF NOT EXISTS apps (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    platform_id BIGINT DEFAULT 1,
    tenant_id VARCHAR(100) DEFAULT '1',
    created_by VARCHAR(100),
    updated_by VARCHAR(100),
    framework VARCHAR(50) DEFAULT 'Vue',
    home_page BIGINT,
    page_content JSON,
    global_state JSON,
    data_source_global JSON,
    extend_config JSON,
    published BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_tenant (tenant_id),
    INDEX idx_platform (platform_id),
    INDEX idx_name (name)
);

CREATE TABLE IF NOT EXISTS pages (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    app_id BIGINT NOT NULL,
    route VARCHAR(255) NOT NULL,
    page_desc TEXT,
    page_content JSON,
    is_home BOOLEAN DEFAULT FALSE,
    is_body BOOLEAN DEFAULT FALSE,
    parent_id BIGINT,
    group_name VARCHAR(100),
    is_page BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_app (app_id),
    INDEX idx_route (route),
    INDEX idx_parent (parent_id),
    UNIQUE KEY uk_app_route (app_id, route)
);

CREATE TABLE IF NOT EXISTS page_histories (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    page_id BIGINT NOT NULL,
    message TEXT,
    page_content JSON NOT NULL,
    is_body BOOLEAN DEFAULT FALSE,
    is_home BOOLEAN DEFAULT FALSE,
    group_name VARCHAR(100),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_page (page_id),
    INDEX idx_created (created_at)
);

CREATE TABLE IF NOT EXISTS blocks (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    label VARCHAR(255),
    description TEXT,
    content JSON NOT NULL,
    category_id BIGINT,
    app_id BIGINT,
    created_by VARCHAR(100),
    published BOOLEAN DEFAULT FALSE,
    version VARCHAR(50),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_category (category_id),
    INDEX idx_app (app_id),
    INDEX idx_name (name)
);

CREATE TABLE IF NOT EXISTS block_categories (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    parent_id BIGINT,
    sort INT DEFAULT 0,
    block_count INT DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_parent (parent_id)
);

CREATE TABLE IF NOT EXISTS block_groups (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    blocks JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS i18n_entries (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    app_id BIGINT NOT NULL,
    lang VARCHAR(20) NOT NULL,
    key_path VARCHAR(255) NOT NULL,
    value TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY uk_app_lang_key (app_id, lang, key_path),
    INDEX idx_app (app_id),
    INDEX idx_lang (lang)
);

CREATE TABLE IF NOT EXISTS data_sources (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    app_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    config JSON NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_app (app_id)
);
