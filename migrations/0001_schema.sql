CREATE TYPE bookingstatus AS ENUM ('accepted', 'pending', 'rejected');

CREATE TABLE Classrooms(
    class_id SERIAL PRIMARY KEY,
    class_name VARCHAR(30),
    class_number INTEGER
);

CREATE TABLE Bookings(
    booking_id SERIAL PRIMARY KEY,
    class_id INTEGER REFERENCES Classrooms(class_id) NOT NULL,
    booking_from TIMESTAMP NOT NULL,
    booking_to TIMESTAMP NOT NULL,
    booking_owner VARCHAR(80) NOT NULL,
    booking_confirmed bookingstatus NOT NULL,
    booking_delete_hash VARCHAR(20) NOT NULL
);

CREATE TABLE Administrators(
    admin_id SERIAL PRIMARY KEY,
    admin_name VARCHAR(80),
    admin_login VARCHAR(30) NOT NULL,
    admin_password VARCHAR(200) NOT NULL
);
