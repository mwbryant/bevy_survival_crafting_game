# Bevy Survival Crafting

A survival crafting game in [Bevy](https://bevyengine.org/) designed after Don't Starve.

The player can move with WASD and pickup items with Space.  These will be placed into your inventory on the bottom bar of the screen.  If you gather the correct materials (flint and twigs) then some of the tools will highlight in the crafting menu on the left of the screen which can be crafted by clicking on their icons.  A tool can then be equiped by clicking on it in the inventory which will move it to the equiped slot in the bottom right.  Finally if you have equipped an axe then you are able to cut down the trees.

![Example Gif](gifs/survival_demo.gif)

Crafting recipes and SpriteSheet descriptions are loaded from a ron file at run time allowing for easy creation of new recipes without needing to recompile the game.

The game features a custom shader and material to generate the fire effect which lights up the world.  Currently modifying the fire entity in the inspector allows you to increase the size and position of the 3 demo fires but the system supports any number of fires (until GPU limitations take over).

All code is either contributed by community members or was live recorded and commentated in the [Bevy Longs series by LogicProjects on Youtube](https://www.youtube.com/watch?v=w7UVSF4lTj0&list=PLT_D88-MTFOMtJPkMvWzTedfUo5W7oiNH)

Art provided by [Sal](https://github.com/Salzimus)

# Usage

```
cargo run --release
```

# Contributing

Yes please! Any issues, bug fixes, code style fixes are welcome.  This is intended to be an educational project and I'm hoping it's a good example of using Bevy to make a simple game.
