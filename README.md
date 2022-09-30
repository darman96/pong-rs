# pong-rs
A pong clone written in Rust.

# GDD

* Score count for two players
  * Some limit as win condition
* Two Paddles left and right on the screen
  * Controllable via keyboard (up/down, w/s)
  * Must not move outside the screen
* A ball that starts off in the center of the screen
  * Is launched left or right once the game starts
  * Collides with Paddles and is bounced back to the other side
  * If it leaves the screen on the side of one player, increase score of other player and reset ball