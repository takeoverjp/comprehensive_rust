mod button;
mod label;
mod window;

pub trait Widget {
    /// `self` の自然な幅。
    fn width(&self) -> usize;

    /// ウィジェットをバッファに描画します。
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// ウィジェットを標準出力に描画します。
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub use button::Button;
pub use label::Label;
pub use window::Window;
