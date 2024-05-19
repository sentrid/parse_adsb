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

DELIMITER $$
CREATE DEFINER=`root`@`localhost` PROCEDURE `insert_aircraft`(
  `archive_id` int,
  `hex` text,
  `type` text,
  `flight` text,
  `r` text,
  `t` text,
  `alt_baro` int,
  `alt_geom` int,
  `gs` int,
  `track` int,
  `geom_rate` int,
  `category` text,
  `nav_qnh` int,
  `nav_altitude_mcp` int,
  `nav_heading` int,
  `lat` float,
  `lon` float,
  `nic` int,
  `rc` int,
  `seen_pos` int,
  `version` int,
  `nic_baro` int,
  `nac_p` int,
  `nac_v` int,
  `sil` int,
  `sil_type` text,
  `gva` int,
  `sda` int,
  `alert` int,
  `spi` int,
  `messages` int,
  `seen` int,
  `rssi` int
  )
BEGIN

INSERT INTO `adsb`.`aircraft` (`archive_id`, `id`, `hex`, `type`, `flight`, `r`, `t`, `alt_baro`, `alt_geom`, `gs`, `track`, `geom_rate`, `category`, `nav_qnh`, `nav_altitude_mcp`, `nav_heading`, `lat`, `lon`, `nic`, `rc`, `seen_pos`, `version`, `nic_baro`, `nac_p`, `nac_v`, `sil`, `sil_type`, `gva`, `sda`, `alert`, `spi`, `messages`, `seen`, `rssi`)
VALUES (archive_id, id, hex, type, flight, r, t, alt_baro, alt_geom, gs, track, geom_rate, category, nav_qnh, nav_altitude_mcp, nav_heading, lat, lon, nic, rc, seen_pos, version, nic_baro, nac_p, nac_v, sil, sil_type, gva, sda, alert, spi, messages, seen, rssi);


END$$
DELIMITER ;
SELECT * FROM software_license.account;
DELETE FROM software_license.account WHERE account_row_id = 6;DELIMITER $$
CREATE DEFINER=`root`@`localhost` PROCEDURE `get_account_by_id`(
	account_id_value varchar(45)
)
BEGIN
	SELECT * FROM account WHERE account_id = account_id_value;
END$$
DELIMITER ;

DELIMITER $$
CREATE DEFINER=`root`@`localhost` PROCEDURE `insert_archive`(
	now DECIMAL,
    messages bigint
)
BEGIN
	INSERT INTO `adsb`.`archive`(`now`, `messages`) VALUES (now, messages);
    SELECT LAST_INSERT_ID() as id;
END$$
DELIMITER ;
