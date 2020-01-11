-- add `photo_stats` view
-- this view simply groups photos by their rating and sums them

CREATE VIEW photos_stats AS
SELECT UNRATED,
       (UNRATED / TOTAL_KEPT) * 100              UNRATED_PERCENT,
       HIDDEN,
       (HIDDEN / TOTAL_KEPT) * 100               HIDDEN_PERCENT,
       NEUTRAL,
       (NEUTRAL / TOTAL_KEPT) * 100              NEUTRAL_PERCENT,
       WALLPAPER_CANDIDATES,
       (WALLPAPER_CANDIDATES / TOTAL_KEPT) * 100 WC_PERCENT,
       FAVORITES,
       (FAVORITES / TOTAL_KEPT) * 100            FAVORITES_PERCENT,
       TOTAL_KEPT,
       (TOTAL_KEPT / TOTAL) * 100                KEPT_PERCENT,
       PENDING_DELETE,
       (PENDING_DELETE / TOTAL) * 100            DELETE_PERCENT,
       TOTAL
FROM (
         SELECT (SELECT COUNT(*)
                 from photos
                 where rating = 0)  UNRATED,
                (SELECT COUNT(*)
                 from photos
                 where rating = 1)  PENDING_DELETE,
                (SELECT COUNT(*)
                 from photos
                 where rating = 2)  HIDDEN,
                (SELECT COUNT(*)
                 from photos
                 where rating = 3)  NEUTRAL,
                (SELECT COUNT(*)
                 from photos
                 where rating = 4)  WALLPAPER_CANDIDATES,
                (SELECT COUNT(*)
                 from photos
                 where rating = 5)  FAVORITES,
                (SELECT COUNT(*)::decimal
                 from photos
                 where rating <> 1) TOTAL_KEPT,
                (SELECT COUNT(*)::decimal
                 from photos_all)   TOTAL) s;

select *
from photos
where rating <> 1;