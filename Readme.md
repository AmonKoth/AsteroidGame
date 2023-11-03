Performance aware asteroid shooter:
This project is done with Rust lang. using SDL library's rendering abilities.
The aim of this project is to create an Asteroids game that is performance aware.
The art assets that I use for this project are spritesheets of an arcade game called Metal Slug 3

This project is for educational purposes only.

First Release:
The game has been created using SDL and it's two additional features, tff and image. One of the requirements for this project was to have ecs in it, for this purpose I used speccs and speccsdrive crate.
I did this project in Rust since I wanted to start learning it for a while and this was a good opportunity.
Components in the first itteration are: 
Position,
Renderable,
Player,
Asteroid,
Rocket and
GameData to keep the score 

Movement is : W-A-S-D to move, Mouse to aim Space to shoot

Second Release:

Important optimizations:

Created a texture manager: Created the entity textures at the start and keept them in a hashmap since the game was dropping frames after 10 entities, and called the textures from said hashmap. The reason for this is sdl's create textures funciton is an heavy function and this workaround had to be done.

Changed the way create UI textures: The function needed to create UI in sdl, create_surface_from_textures, is also an expensive operation. And since you clear the screen every time you want to render something new, they have to exist in every frame also, they did not have the SDL_UpdateRect function in here, you could not do partial rendering. To fix this what I did is to keep all the UI textures in a list and update this list, by clearing it and recrateing it, every 100 iteration of the loop. For this fix the game improved about 20 frames, which is an equivelent of 20 000 entities.

After this point my performance issues were mostly caused by rendering function of sdl(canvas.copy_ex) and changing the funtionilty of this defeats the purpose of using sdl so I ignored it for the rest of the project.

Another issue was I did not liked how my collision checking works at the moment. The collisions are checking every entity, like player and rocket, and compare the position for every asteroid in the scene. Although I have at most 1 player and 5 rockets available at the same time I did not liked this method for collision detection since its quadratic time complexity. To fix this I implamented grid method to the game. Every object now has an Collider entity which keeps track of the entities grid. And these only check collision for the entities in the same grid meaning if their grid is not equal, collision will imidiatly return, by doing so I reduced the time complexity of the most entities in the same, which are not on the same gird with the target of the collision checking, will have linear time complexity, for most of the objects, in this case constant since I have 1 player and at most 5 rockets. However this implemantation is very basis level and does not take account the fact of the asteroids size. In order to not have glitches in this release I have temporarily disabled this.