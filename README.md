# Tower game

Simple TPS game with Bevy 0.13. Move with WASD, dash with SPACE, look around, zoom in and out with the mouse wheel, left click to shoot.

Shots and dashes have cooldowns (not visible yet). Shots produce physical bullets that collide with the environment and the enemy.

If a bullet touches the enemy box, it deals one damage point. The enemy has a total health equal to the number of the level. When the enemy dies, a new level is created right above the previous one, with a new hue-shifted color. The player is then teleported to the next level. There is no level limit for now.

![game image](level1.png)