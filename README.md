Asteroid roulette
=================

This is a simulator for the Asteroid roulette game found in "Gobsmacking Galaxy
(The Knowledge)" by Kjartan Poskitt.

Usage
-----

    cargo run

Testing
-------

    cargo test

Documentation
-------------

    cargo doc

Get win/loss probabilities
--------------------------

This command will report win/loss statistics after 100 000 games.

    cargo test victory_probabilities -- --nocapture
