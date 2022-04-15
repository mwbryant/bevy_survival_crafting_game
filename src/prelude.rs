use crate::assets;
pub use assets::{GameAssetsPlugin, PlaceHolderGraphics, RESOLUTION};

use crate::crafting;
pub use crafting::CraftingPlugin;

use crate::game_camera;
pub use game_camera::{GameCamera, GameCameraPlugin};

use crate::inventory;
pub use inventory::{give_inventory_item, take_inventory_item, Inventory, InventoryPlugin};

use crate::player;
pub use player::{Player, PlayerPlugin};

use crate::item;
pub use item::{ItemAndCount, ItemType, ItemsPlugin, Pickupable};

use crate::mouse;
pub use mouse::{MousePlugin, MousePosition};
