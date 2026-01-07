//! ゲーム全体の描画命令をレンダーに送る.

use crate::gameplay::{
    block::block_datas::{self, BlockType}, field, game_manager::{GameManager, GameState, PlayStyle, TitleChoice}, game_renderer_sender::GameRendererSender, gameplay_manager::GameplayManager
};
use crate::utility::grid::Grid;
use crate::console_renderer::render_manager::RenderQueueData;
use crate::RENDER_MANAGER;
use crossterm::style::Color;
use std::collections::VecDeque;

const GAMEPLAY_WIDTH: i32 = 60;

pub struct GameSender {
}

impl GameSender {
    pub fn new() -> Self {
        GameSender {
        }
    }

    fn make_playing_queues(&self, game: &GameManager) -> VecDeque<RenderQueueData> {
        let mut queues = VecDeque::new();
        for (index, gameplay) in game.gameplay_managers.iter().enumerate() {
            let gameplay_sender = GamePlaySender::new(Grid::new(10 + GAMEPLAY_WIDTH * index as i32, 0));
            queues.append(&mut gameplay_sender.gameplay_sender(gameplay));
        }
        queues
    }

    fn calc_center_pos_to_left_pos(&self, x: i32, str: &String) -> i32{
        let str_width: i32 = str.chars().map(|char| if char.is_ascii() { 1 } else { 2 } ).sum();
        let result = x - str_width / 2;
        if result < 0 {0} else {result}
    }
}

impl GameRendererSender for GameSender {
    fn game_sender(&self, game: &GameManager) {
        let mut queues = VecDeque::new();
        match game.get_state() {
            GameState::Title => {
                let title_center_pos_x = 20;
                let title_str = String::from("落ちものパズルゲーム");
                let start_str = String::from(match game.get_title_choice_command() {
                    TitleChoice::Play => {
                        match game.get_play_style() {
                            PlayStyle::Solo => "-1人でプレイ- NPCとプレイ  2人でプレイ ",
                            PlayStyle::WithNPC(_) => " 1人でプレイ -NPCとプレイ- 2人でプレイ ",
                            PlayStyle::VSPlayer => " 1人でプレイ  NPCとプレイ -2人でプレイ-",
                        }
                    },
                    TitleChoice::Exit => " 1人でプレイ  NPCとプレイ  2人でプレイ ",
                });
                let exit_str = String::from(match game.get_title_choice_command() {
                    TitleChoice::Play => "やめる",
                    TitleChoice::Exit => "-やめる-",
                });
                let high_score_str = format!("現在のハイスコア：{:>10}", game.get_high_score());
                // TODO: コマンドをkey_code_to_console_key_codeなどから取得して表示する.
                let tutorial_str = String::from("操作：↑↓→←キー　決定：Enter");
                let gameplay_tutorial_str = String::from("1Pブロック操作：wasd 回転：zx ホールド:c ドロップ：f ポーズ：r");
                let vs_tutorial_str = String::from("2Pブロック操作：ijkl 回転：,m ホールド:. ドロップ：; ポーズ：p");

                queues.push_back(RenderQueueData::new(Grid::new(self.calc_center_pos_to_left_pos(title_center_pos_x, &title_str), 8), 
                                            title_str, Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(self.calc_center_pos_to_left_pos(title_center_pos_x, &high_score_str), 11), 
                                            high_score_str, Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(self.calc_center_pos_to_left_pos(title_center_pos_x, &start_str), 13), 
                                            start_str, Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(self.calc_center_pos_to_left_pos(title_center_pos_x, &exit_str), 14), 
                                            exit_str, Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(self.calc_center_pos_to_left_pos(title_center_pos_x, &tutorial_str), 16),
                                            tutorial_str, Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(self.calc_center_pos_to_left_pos(title_center_pos_x, &gameplay_tutorial_str), 17),
                                            gameplay_tutorial_str, Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(self.calc_center_pos_to_left_pos(title_center_pos_x, &vs_tutorial_str), 18),
                                            vs_tutorial_str, Color::White));
            },
            GameState::Playing => {
                queues.append(&mut self.make_playing_queues(game));
            },
            GameState::Paused => {
                queues.append(&mut self.make_playing_queues(game));
                queues.push_back(RenderQueueData::new(Grid::new(35,9), String::from("　　　　　"), Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(35,10), String::from("　ポーズ　"), Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(35,11), String::from("　　　　　"), Color::White));
            },
            GameState::GameOver => {
                queues.append(&mut self.make_playing_queues(game));
                queues.push_back(RenderQueueData::new(Grid::new(33,9), String::from("　　　　　　　　　"), Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(33,10), String::from("　ゲームオーバー　"), Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(33,11), String::from("　　　　　　　　　"), Color::White));
                if game.get_high_score_updated() {
                    queues.push_back(RenderQueueData::new(Grid::new(33,12), String::from("　ハイスコア更新　"), Color::White));
                    queues.push_back(RenderQueueData::new(Grid::new(33,13), String::from("　　　　　　　　　"), Color::White));
                }
                queues.push_back(RenderQueueData::new(Grid::new(33,12), String::from(" 　press Enter　 "), Color::White));
                queues.push_back(RenderQueueData::new(Grid::new(33,13), String::from("　　　　　　　　　"), Color::White));
            },
        };
        let mut render_manager = RENDER_MANAGER.lock().unwrap();
        render_manager.push_queues(&mut queues);
    }
}

struct GamePlaySender {
    pos: Grid,
}

impl GamePlaySender {
    pub fn new(pos: Grid) -> Self {
        GamePlaySender {
            pos: pos,
        }
    }

    fn make_cells_queues(&self, cells: &[Vec<BlockType>], window_width: usize, window_height: usize, start_pos: &Grid, force_color: Option<Color>) -> Result<VecDeque<RenderQueueData>, &'static str> {
        let mut queues = VecDeque::new();
        if window_height < cells.len() || window_width < cells[0].len() {
            // エラー.枠サイズがおかしい.
            return Err("枠サイズがおかしい");
        }
        let base_color = if let Some(color) = force_color {color} else {Color::White};
        let mut write_height = 0;
        let header_height = (window_height - cells.len() + 1) / 2;
        let mut header_string = String::from("┏");
        for _ in 0..window_width {
            header_string += "━━";
        }
        header_string += "┓";
        queues.push_back(RenderQueueData::new(Grid::new(0,write_height) + start_pos, header_string, base_color));
        write_height += 1;
        for _ in 0..header_height {
            header_string = String::from("┃");
            for _ in 0..window_width {
                header_string += "　";
            }
            header_string += "┃";
            queues.push_back(RenderQueueData::new(Grid::new(0,write_height) + start_pos, header_string, base_color));
            write_height += 1;
        }

        let buffer_width: usize = window_width - cells[0].len();
        let oneside_buffer_width = buffer_width / 2;
        let use_harf_buffer = buffer_width % 2 == 1;
        let mut render_color = base_color;
        let mut render_string ;
        // queueの削減のため、同じ色の文字列はまとめて投げるようにする.
        for cells_line in cells.iter() {
            let mut render_position_x = 0;
            let mut next_position_x = 0;
            render_string = String::from("┃");
            next_position_x += 1;
            if use_harf_buffer {
                render_string.push_str(" ");
                next_position_x += 1;
            }
            for _ in 0..oneside_buffer_width {
                render_string.push_str("　");
                next_position_x += 2;
            }
            for block_cell in cells_line.iter() {
                let cell_color = if let Some(color) = force_color {color} else {get_block_color(*block_cell)};
                if render_color != cell_color {
                    queues.push_back(RenderQueueData::new(Grid::new(render_position_x, write_height) + start_pos, render_string, render_color));
                    render_string = String::from("");
                    render_color = cell_color;
                    render_position_x = next_position_x;
                }
                if *block_cell == BlockType::None {
                    render_string += "　";
                }
                else {
                    render_string += "ロ";
                }
                next_position_x += 2;
            }
            if render_color != base_color {
                queues.push_back(RenderQueueData::new(Grid::new(render_position_x, write_height) + start_pos, render_string, render_color));
                render_string = String::from("");
                render_color = base_color;
                render_position_x = next_position_x;
            }
            for _ in 0..oneside_buffer_width {
                render_string.push_str("　");
            }
            if use_harf_buffer {
                render_string.push_str(" ");
            }
            render_string += "┃";
            queues.push_back(RenderQueueData::new(Grid::new(render_position_x, write_height) + start_pos, render_string, render_color));
            write_height += 1;
        }

        let footer_height = (window_height - cells.len()) / 2;
        let mut footer_string;
        for _ in 0..footer_height {
            footer_string = String::from("┃");
            for _ in 0..window_width {
                footer_string += "　";
            }
            footer_string += "┃";
            queues.push_back(RenderQueueData::new(Grid::new(0, write_height) + start_pos, footer_string, base_color));
            write_height += 1;
        }
        footer_string = String::from("┗");
        for _ in 0..window_width {
            footer_string += "━━";
        }
        footer_string += "┛";
        queues.push_back(RenderQueueData::new(Grid::new(0, write_height) + start_pos, footer_string, base_color));
        Ok(queues)
    }

    fn make_block_queues(&self, block_type: BlockType, start_pos: &Grid) -> VecDeque<RenderQueueData>{
        let block = block_datas::block_shape(block_type);
        let window_width = 5;
        let window_height = 5;
        self.make_cells_queues(&block, window_width, window_height, start_pos, None).expect("ブロック書き込みに失敗")
    }

    fn make_raw_block_queues(&self, block: &Vec<Vec<BlockType>>, color: Color, start_pos_left_bottom: &Grid) -> VecDeque<RenderQueueData>{
        let mut queues = VecDeque::new();
        let start_pos_left_top = Grid::new(start_pos_left_bottom.x, start_pos_left_bottom.y + 1 - block.len() as i32);
        for y in 0..block.len() {
            for x in 0..block[y].len() {
                let cell_type = block[y][x];
                if cell_type != BlockType::None {
                    let pos = Grid::new(x as i32 * 2, y as i32) + &start_pos_left_top;
                    queues.push_back(RenderQueueData::new(pos, String::from("ロ"), color));
                }            
            }
        }
        queues
    }

    pub fn gameplay_sender(&self, gameplay: &GameplayManager) ->VecDeque<RenderQueueData> {
        let mut queues = VecDeque:: new();
        // ホールドブロックの表示
        let hold_pos = Grid::new(7, 1) + &self.pos;
        queues.append(&mut self.make_block_queues(gameplay.get_hold_block(), &hold_pos));

        // スコアとステータスの表示.
        let score_pos_x = 1;
        let mut score_pos_y = 10;
        let stats = gameplay.get_stats();
        {
            let render_string = format!("SCORE:   {:>10}", gameplay.get_score());
            queues.push_back(RenderQueueData::new(Grid::new(score_pos_x, score_pos_y) + &self.pos, render_string, Color::White));
            score_pos_y += 2;
            let render_string = format!("LINES:     {: >8}", stats.erace_lines);
            queues.push_back(RenderQueueData::new(Grid::new(score_pos_x, score_pos_y) + &self.pos, render_string, Color::White));
            score_pos_y += 1;
            let render_string = format!{"LEVEL:     {: >8}", stats.level};
            queues.push_back(RenderQueueData::new(Grid::new(score_pos_x, score_pos_y) + &self.pos, render_string, Color::White));
            score_pos_y += 1;
            let render_string = format!("MAX_ERACE: {: >8}", stats.max_erace_count);
            queues.push_back(RenderQueueData::new(Grid::new(score_pos_x, score_pos_y) + &self.pos, render_string, Color::White));
            score_pos_y += 1;
            let render_string = format!("T-SPINS:   {: >8}", stats.t_spin_erace_lines);
            queues.push_back(RenderQueueData::new(Grid::new(score_pos_x, score_pos_y) + &self.pos, render_string, Color::White));
            score_pos_y += 1;
            let render_string = format!("COMBOS:    {: >8}", stats.combos);
            queues.push_back(RenderQueueData::new(Grid::new(score_pos_x, score_pos_y) + &self.pos, render_string, Color::White));
        }

        // フィールドの表示.
        // コントロールブロック描画のために、フィールドの上部を少し空けておく.
        let field_pos = Grid::new(20, 3) + &self.pos;
        let field_pos_except_frame = Grid::new(field_pos.x + 1, field_pos.y + 1);
        queues.append(&mut self.make_cells_queues(&gameplay.get_field_data()[block_datas::BLOCK_START_POSITION_Y as usize..], field::FIELD_WIDTH, field::FIELD_HEIGHT_WITH_OUTSIDE - block_datas::BLOCK_START_POSITION_Y as usize
                            , &field_pos, if gameplay.is_game_over() {Some(Color::Grey)} else {None}).expect("フィールド書き込みに失敗"));

        // 影の表示.
        let mut ghost_pos = gameplay.get_ghost_pos();
        ghost_pos.x *= 2;
        ghost_pos.y -= block_datas::BLOCK_START_POSITION_Y;
        ghost_pos = ghost_pos + &field_pos_except_frame;
        queues.append(&mut self.make_raw_block_queues(&gameplay.get_control_block().block, Color::Grey, &ghost_pos));

        // コントロールブロックの表示.
        let mut control_block_pos = gameplay.get_control_block().position.clone();
        control_block_pos.x *= 2;
        control_block_pos.y -= block_datas::BLOCK_START_POSITION_Y;
        control_block_pos = control_block_pos + &field_pos_except_frame;
        queues.append(&mut self.make_raw_block_queues(&gameplay.get_control_block().block, get_block_color(gameplay.get_control_block().block_type), &control_block_pos));

        // 次のブロックの表示.
        let mut next_blocks_pos = Grid::new(43, 0) + &self.pos;
        let next_block_margin = Grid::new(0, 8);
        let disp_next_block_count = 3;
        for i in 0..disp_next_block_count {
            queues.append(&mut self.make_block_queues(gameplay.get_next_block(i), &next_blocks_pos));
            next_blocks_pos = next_blocks_pos + &next_block_margin;
        }
        queues
    }
}

fn get_block_color(block_type: BlockType) -> Color {
    match block_type {
        BlockType::I => Color::Cyan,
        BlockType::J => Color::Blue,
        BlockType::L => Color::Magenta,
        BlockType::T => Color::Rgb { r: (255), g: (0), b: (255) },
        BlockType::Attacked => Color::Rgb { r: {128}, g: {128}, b: {128}},
        BlockType::None => Color::White,
    }
}
