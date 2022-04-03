use crate::assets;
pub use assets::{RESOLUTION, GameAssetsPlugin, PlaceHolderGraphics};

use crate::crafting;
pub use crafting::CraftingPlugin;

use crate::game_camera;
pub use game_camera::{GameCamera, GameCameraPlugin};

use crate::inventory;
pub use inventory::{Inventory, InventoryPlugin, give_inventory_item, take_inventory_item};

use crate::player;
pub use player::{Player, PlayerPlugin};

use crate::item;
pub use item::{ItemType, ItemAndCount, Pickupable, ItemsPlugin };

