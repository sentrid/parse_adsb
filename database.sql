CREATE TABLE generic (
                         id INT NOT NULL AUTO_INCREMENT,
                         now INT,
                         messages INT,
                         PRIMARY KEY (id)
);

CREATE TABLE generic_aircraft (
                                  generic_id INT,
                                  id INT NOT NULL AUTO_INCREMENT,
                                  hex TEXT,
                                  type TEXT,
                                  flight TEXT,
                                  r TEXT,
                                  t TEXT,
                                  alt_baro INT,
                                  alt_geom INT,
                                  gs INT,
                                  track INT,
                                  geom_rate INT,
                                  category TEXT,
                                  nav_qnh INT,
                                  nav_altitude_mcp INT,
                                  nav_heading INT,
                                  lat INT,
                                  lon INT,
                                  nic INT,
                                  rc INT,
                                  seen_pos INT,
                                  version INT,
                                  nic_baro INT,
                                  nac_p INT,
                                  nac_v INT,
                                  sil INT,
                                  sil_type TEXT,
                                  gva INT,
                                  sda INT,
                                  alert INT,
                                  spi INT,
                                  messages INT,
                                  seen INT,
                                  rssi INT,
                                  PRIMARY KEY (id),
                                  FOREIGN KEY (generic_id) REFERENCES generic(id)
);

CREATE TABLE generic_aircraft_nav_modes (
                                            generic_aircraft_id INT,
                                            id INT NOT NULL AUTO_INCREMENT,
                                            value TEXT,
                                            PRIMARY KEY (id),
                                            FOREIGN KEY (generic_aircraft_id) REFERENCES generic_aircraft(id)
);

CREATE TABLE generic_aircraft_mlat (
                                       generic_aircraft_id INT,
                                       id INT NOT NULL AUTO_INCREMENT,
                                       value TEXT,
                                       PRIMARY KEY (id),
                                       FOREIGN KEY (generic_aircraft_id) REFERENCES generic_aircraft(id)
);

CREATE TABLE generic_aircraft_tisb (
                                       generic_aircraft_id INT,
                                       id INT NOT NULL AUTO_INCREMENT,
                                       value TEXT,
                                       PRIMARY KEY (id),
                                       FOREIGN KEY (generic_aircraft_id) REFERENCES generic_aircraft(id)
);