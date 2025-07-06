//! Systems module for the sidescroller game
//!
//! This module organiza todos os sistemas do jogo em grupos lógicos:
//! - Setup: Sistemas para inicialização do mundo do jogo
//! - Movement: Sistemas para movimentação do jogador e física
//! - Animation: Sistemas para animações de sprites e efeitos visuais
//! - Debug: Sistemas para depuração e ferramentas de desenvolvimento

pub mod animation;
pub mod debug;
pub mod movement;
pub mod setup;

// Re-export commonly used systems for easier importing
pub use animation::{execute_animations, update_animation_state};
pub use debug::{debug_tile_collisions, debug_tile_grid, debug_tile_info, debug_tileset_info, toggle_debug_render};
pub use movement::{move_player, update_facing_direction};
pub use setup::{setup_graphics, setup_physics};
