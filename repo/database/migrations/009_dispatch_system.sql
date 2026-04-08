-- Station zones within the store
CREATE TABLE IF NOT EXISTS station_zones (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    zone_type ENUM('Espresso','ColdBrew','Tea','BlendedDrinks','Cashier','General') NOT NULL,
    max_concurrent_tasks INT NOT NULL DEFAULT 3,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Staff shift windows
CREATE TABLE IF NOT EXISTS shift_windows (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    zone_id BIGINT NOT NULL,
    shift_date DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (zone_id) REFERENCES station_zones(id) ON DELETE CASCADE,
    INDEX idx_shift_date (shift_date, start_time),
    INDEX idx_shift_user (user_id, shift_date)
);

-- Staff reputation scores
CREATE TABLE IF NOT EXISTS staff_reputation (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL UNIQUE,
    total_tasks_completed INT NOT NULL DEFAULT 0,
    avg_completion_time_secs INT NOT NULL DEFAULT 0,
    quality_score DECIMAL(3,2) NOT NULL DEFAULT 5.00,
    reliability_score DECIMAL(3,2) NOT NULL DEFAULT 5.00,
    composite_score DECIMAL(5,2) NOT NULL DEFAULT 50.00,
    last_updated DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Task assignments
CREATE TABLE IF NOT EXISTS task_assignments (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    order_id BIGINT NOT NULL,
    assigned_to BIGINT NULL,
    zone_id BIGINT NULL,
    dispatch_mode ENUM('Grab','Assigned') NOT NULL DEFAULT 'Grab',
    status ENUM('Queued','Offered','Accepted','InProgress','Completed','Rejected','Expired') NOT NULL DEFAULT 'Queued',
    priority INT NOT NULL DEFAULT 50,
    offered_at DATETIME NULL,
    accepted_at DATETIME NULL,
    started_at DATETIME NULL,
    completed_at DATETIME NULL,
    offer_expires_at DATETIME NULL,
    notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
    FOREIGN KEY (assigned_to) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (zone_id) REFERENCES station_zones(id) ON DELETE SET NULL,
    INDEX idx_task_status (status, priority DESC),
    INDEX idx_task_assignee (assigned_to, status),
    INDEX idx_task_zone (zone_id, status),
    UNIQUE INDEX idx_task_order (order_id)
);

-- Dispatch configuration
CREATE TABLE IF NOT EXISTS dispatch_config (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    config_key VARCHAR(100) NOT NULL UNIQUE,
    config_value VARCHAR(500) NOT NULL,
    description TEXT,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Seed dispatch config
INSERT INTO dispatch_config (config_key, config_value, description) VALUES
    ('default_mode', 'Grab', 'Default dispatch mode: Grab or Assigned'),
    ('offer_timeout_secs', '30', 'Seconds before an offered task expires'),
    ('max_concurrent_per_staff', '3', 'Max tasks one staff member can handle simultaneously'),
    ('enable_reputation_weighting', 'true', 'Use reputation scores in assignment ranking'),
    ('grab_queue_visible_count', '10', 'Number of tasks visible in grab queue')
ON DUPLICATE KEY UPDATE config_value = VALUES(config_value);

-- Seed station zones
INSERT INTO station_zones (name, description, zone_type, max_concurrent_tasks) VALUES
    ('Espresso Bar', 'Main espresso machine station', 'Espresso', 4),
    ('Cold Brew Station', 'Cold brew and iced drink preparation', 'ColdBrew', 3),
    ('Tea Corner', 'Hot and iced tea preparation', 'Tea', 3),
    ('Blended Station', 'Smoothies and blended drinks', 'BlendedDrinks', 2),
    ('Front Counter', 'Order pickup and cashier', 'Cashier', 2);
