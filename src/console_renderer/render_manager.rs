//! 指定された文字をコンソールに書き込む.
 
use crate::utility::grid::Grid;
use std::collections::VecDeque;
use std::io::{stdout, Write};
use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    execute, queue, 
    style::{Color, Print, SetForegroundColor},
};

/// コンソールに書き込む命令の構造体.
pub struct RenderQueueData {
    pub grid: Grid,
    pub disp_string: String,
    pub color: Color,
}
impl RenderQueueData {
    pub fn new(grid: Grid, disp_string: String, color: Color) -> Self {
        RenderQueueData{grid, disp_string, color}
    }
}
/// コンソールに書き込んだりする構造体.
pub struct RenderManager {
    render_queue: VecDeque<RenderQueueData>,
}

impl RenderManager {
    /// 新規インスタンス作成.
    /// 基本的にはシングルトンを想定している.
    pub fn new() -> Self {
        RenderManager{
            render_queue: VecDeque::new(),
        }
    }

    pub fn push_queues(&mut self, queues: &mut VecDeque<RenderQueueData>){
        self.render_queue.append(queues);
    }

    /// 書き込み命令を追加.
    pub fn push_queue(&mut self, queue: RenderQueueData) {
        if queue.grid.x < 0 || queue.grid.x > u16::MAX as i32 || queue.grid.y < 0 || queue.grid.y > u16::MAX  as i32{
            eprintln!("push queue grid data error {:?}", queue.grid);
            return;
        }
        self.render_queue.push_back(queue);
    }
    /// コンソール画面のクリア.
    pub fn clear(&self) {
        execute!(
            stdout(),
            cursor::Hide,
            Clear(ClearType::All)
        ).unwrap();
    }
    /// 追加された書き込み命令に従って書き込み.
    pub fn render(&mut self) {
        let mut stdout = stdout();
        while let Some(data) = self.render_queue.pop_front() {
            let _ = queue!(
                stdout,
                cursor::MoveTo(data.grid.x as u16, data.grid.y as u16),
                SetForegroundColor(data.color),
                Print(data.disp_string),
            );
        }
        let _ = stdout.flush();
    }
}
