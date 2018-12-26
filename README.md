Asteroid roulette
=================

    $ asteroidroulette
    You rolled 6, 6...!
    You died.
    You got hit by Ceres.

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

Example game
------------
    $ asteroidroulette
    You rolled 5, 5...!
    Your engine turned off!
    You rolled 1...!
    You would have moved to 1, but your engines won't turn on.
    You rolled 5...!
    Your engine turned back on!
    You rolled 3, 2, 4, 1...!
    You moved forward to 1.
    You rolled 4, 2...!
    You moved forward to 2.
    You rolled 5, 4, 6, 3...!
    You moved forward to 3.
    You rolled 4...!
    You moved forward to 4.
    You rolled 1, 3, 4, 3, 4, 2, 1, 1...!
    You lost your shields!
    You rolled 2, 1, 3, 5...!
    You moved forward to 5.
    You rolled 6...!
    You moved forward to 6.
    You won!
