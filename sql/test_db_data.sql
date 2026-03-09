INSERT INTO administrators VALUES
(DEFAULT, 'Adam', 'Kowalski', 'tempPassword');

INSERT INTO classrooms VALUES
(0, '', 10),
(1, '', 11),
(2, '', 12),
(3, 'Widowiskowa', 20),
(4, 'Aula', 30);

INSERT INTO bookings VALUES
(DEFAULT, 1, '2026-02-10 14:30:00', '2026-02-10 16:30:00', 'Maciek Józefkowicz', 'accepted', '5755620910692865178'),
(DEFAULT, 1, '2026-02-10 16:30:00', '2026-02-10 18:30:00', 'Suzie', 'pending', '5755620910692865178'),
(DEFAULT, 3, '2026-02-15 10:30:00', '2026-02-15 12:30:00', 'Kasztan Jakiśtam', 'accepted', '5755620910692865178'),
(DEFAULT, 0, '2026-02-13 12:00:00', '2026-02-13 13:30:00', 'Lucy', 'rejected', '5755620910692865178');
