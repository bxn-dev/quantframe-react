## Features

- ✨ Added a minimum profit threshold logic to the live scraper, allowing users to set a minimum profit percentage or flat profit amount for their trades.
- ✨ Added a full Syndicate trading pipeline: a new `syndicate_item` database table and API, a redesigned WTS settings panel with per-syndicate standing tracking and an "ignore standing" toggle, importing syndicate items from Warframe Market, and a live scraper Syndicate tab with sell/edit/delete/export operations.
- ✨ Reworked WF Inventory to support pluggable sources — Warframe profile or AlecaFrame (`lastData.dat` watcher) — selectable in Settings → Advanced → WF Inventory, with a manual refresh button and syndicate standing import.

## Fixes

- 🛠️ Fixed the unit price being recorded as the total trade price instead of the per-item unit price (closes #124).

## Refactors

- ♻️ Reworked the live scraper's selling, wishlist, and syndicate pipelines so the order summary is only logged after a successful Warframe Market order update, and moved the "mark as live" persistence to run only on success.

## Dev Notes

## Icons

- ⏰ TODO
- ✨ Features
- 🛠️ Fixes
- ♻️ Refactors
