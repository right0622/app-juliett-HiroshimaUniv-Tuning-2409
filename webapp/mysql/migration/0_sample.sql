-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
-- users テーブル
CREATE INDEX idx_users_username ON users (username);

-- sessions テーブル
CREATE INDEX idx_sessions_session_token ON sessions (session_token);

-- dispatchers テーブルにインデックスを追加
CREATE INDEX idx_dispatchers_user_id ON dispatchers (user_id);

-- nodesテーブルにインデックスを追加
CREATE INDEX idx_nodes_area_id ON nodes (area_id);

-- edgesテーブルにインデックスを追加
CREATE INDEX idx_edges_node_a_id ON edges (node_a_id);
CREATE INDEX idx_edges_node_b_id ON edges (node_b_id);

-- locationsテーブルにインデックスを追加
CREATE INDEX idx_locations_tow_truck_id ON locations (tow_truck_id);
CREATE INDEX idx_locations_tow_truck_id_timestamp ON locations(tow_truck_id, timestamp DESC);

-- tow_trucksテーブルにインデックスを追加
CREATE INDEX idx_tow_trucks_driver_id ON tow_trucks(driver_id);
CREATE INDEX idx_tow_trucks_area_id ON tow_trucks(area_id);
CREATE INDEX idx_tow_trucks_status ON tow_trucks(status);
