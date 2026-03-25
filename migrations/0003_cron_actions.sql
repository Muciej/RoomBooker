CREATE EXTENSION IF NOT EXISTS pg_cron;

SELECT cron.schedule(
    'cleanup_old_bookings',
    '0 3 * * 0',
    $$DELETE FROM bookings
       WHERE booking_to < NOW() - INTERVAL '7 days'$$
);
