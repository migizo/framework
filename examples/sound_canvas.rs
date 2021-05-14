extern crate rodio;

use std::io::BufReader;
use std::thread;

use rodio::Source;

// canvas
use orbtk::{prelude::*, render::platform::RenderContext2D, utils};
use std::cell::Cell;

static DEFAULT_THEME: &'static str = include_str!("../assets/css/sound_gui_theme.css");

static mut RADIUS: f32 = 100.0;
//============================
// action
//============================
#[derive(Debug, Copy, Clone)]
enum Action {
  // Entityは渡したい値
  ValueChanged(Entity),
  PushButton0,
  PushButton1,
}

//============================
// state
//============================

#[derive(AsAny)]
pub struct MainViewState {
  action: Option<Action>,
  audio_sink: Option<rodio::Sink>,
  test_val: f32,
  start_time: std::time::Instant,
}

impl Default for MainViewState {
  fn default() -> Self {
    MainViewState {
      action: None,
      audio_sink: Some(rodio::Sink::new(
        &rodio::default_output_device().expect("FAILED: default_output_device()"),
      )),
      test_val: 100.0,
      start_time: std::time::Instant::now(),
    }
  }
}

impl MainViewState {
  fn set_action(&mut self, action: impl Into<Option<Action>>) {
    self.action = action.into();
  }
  fn play(&mut self, idx: u32) {
    if let Some(audio_sink) = &self.audio_sink {
      audio_sink.stop();
      // drop(audio_sink);
    }
    // let source = rodio::source::SineWave::new(440).take_duration(Duration::from_millis(5000));
    let device = &rodio::default_output_device().expect("FAILED: default_output_device()");
    let file;
    if idx == 0 {
      // self.test_val += 10.0;
      file = std::fs::File::open("examples/audio/hh_close1.wav").unwrap();
    } else {
      // self.test_val -= 10.0;
      file = std::fs::File::open("examples/audio/snare.wav").unwrap();
    }
    let decode = rodio::Decoder::new(BufReader::new(file)).unwrap();
    self.audio_sink = Some(rodio::Sink::new(&device));
    if let Some(audio_sink) = &self.audio_sink {
      audio_sink.append(decode);
      audio_sink.set_volume(0.5);
    }

    self.start_time = std::time::Instant::now();
  }
}

impl State for MainViewState {
  fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
    // action
    if let Some(action) = self.action {
      match action {
        // Action::ValueChanged(entity) => {
        //   let value = ((*ctx.get_widget(entity).get::<f64>("value")).floor() as f32);
        //   // self.set_volume(value * 0.01);
        // }
        Action::PushButton0 => {
          self.play(0);
        }
        Action::PushButton1 => {
          self.play(1);
        }
        _ => (),
      };
    }
    self.action = None;

    // canvas
    if let Some(canvas) = ctx
      .widget()
      .get_mut::<RenderPipeline>("render_pipeline")
      .0
      .as_any()
      .downcast_ref::<Graphic2DPipeline>()
    {
      canvas.set_time(self.start_time);
    } else {
      println!("missing canvas");
    }
  }
}

//============================
// Graphic2DPipeline
//============================
// OrbTk 2D drawing
#[derive(Clone, PartialEq, Pipeline, Default)]
struct Graphic2DPipeline {
  // 保持したい値をCellで包む
  time: Cell<Option<std::time::Instant>>,
}
impl Graphic2DPipeline {
  fn set_time(&self, val: impl Into<Option<std::time::Instant>>) {
    self.time.set(val.into());
  }
}

impl render::RenderPipeline for Graphic2DPipeline {
  fn draw(&self, render_target: &mut render::RenderTarget) {
    let mut render_context = RenderContext2D::new(render_target.width(), render_target.height());
    // reference: https://developer.mozilla.org/ja/docs/Web/API/CanvasRenderingContext2D
    let width = 640.0;
    let height = 640.0;

    let x = (render_target.width() - width) / 2.0; // center
    let y = (render_target.height() - height) / 2.0; // top
                                                     // let time = (now_time - self.start_time).as_secs_f32();

    // let t = self.time.get().unwrap().elapsed().as_secs_f64();
    let radius;
    unsafe {
      radius = RADIUS;
    }
    render_context.set_stroke_style(utils::Brush::SolidColor(Color::from("#000000")));
    let brush = utils::Brush::LinearGradient {
      start: Point::new(x, y),
      end: Point::new(x + width, y + height),
      stops: vec![
        LinearGradientStop {
          position: 0.0,
          color: Color::from("#0021EB"),
        },
        LinearGradientStop {
          position: 0.5,
          color: Color::from("#CE2F24"),
        },
        LinearGradientStop {
          position: 1.0,
          color: Color::from("#70EF49"),
        },
      ],
    };
    render_context.set_fill_style(brush);
    // render_context.fill_rect(x, y, width, height);
    render_context.begin_path();
    render_context.arc(x + width / 2.0, y + height / 2.0, radius as f64, 0.0, 360.0);
    // render_context.close_path();
    render_context.fill();
    // render_context.stroke();
    render_target.draw(render_context.data());
  }
}

//============================
// mainView
//============================
widget!(MainView<MainViewState> {
  render_pipeline: RenderPipeline
});

impl Template for MainView {
  fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
    self
      .name("MainView")
      .render_pipeline(RenderPipeline(Box::new(Graphic2DPipeline::default())))
      // canvas
      .child(
        Canvas::create()
          .width(640.0)
          .height(800.0)
          .render_pipeline(id)
          .build(ctx),
      )
      // buttons
      .child(
        Grid::create()
          .columns(Columns::create().repeat("*", 6).build())
          .child(
            Button::create()
              .text("Play")
              .class("button")
              .margin((10, 20))
              .vertical_alignment("end")
              .attach(Grid::column(2))
              .attach(Grid::row(47))
              .on_mouse_down(move |states, _| {
                unsafe {
                  RADIUS = 200.0;
                }
                states
                  .get_mut::<MainViewState>(id)
                  .set_action(Action::PushButton0);
                true
              })
              .build(ctx),
          )
          .child(
            Button::create()
              .text("Stop")
              .class("button")
              .margin((10, 20))
              .vertical_alignment("end")
              .attach(Grid::column(3))
              .attach(Grid::row(47))
              .on_mouse_down(move |states, _| {
                unsafe {
                  RADIUS = 10.0;
                }
                states
                  .get_mut::<MainViewState>(id)
                  .set_action(Action::PushButton1);
                true
              })
              .build(ctx),
          )
          .build(ctx),
      )
  }
}

//============================
// main()
//============================

fn main() {
  // use this only if you want to run it as web application.
  orbtk::initialize();

  Application::new()
    .window(|ctx| {
      Window::create()
        .title("OrbTk - minimal example")
        .size(640.0, 800.0)
        // .resizeable(true)
        .theme(
          ThemeValue::create()
            .extension_css(theme::DEFAULT_THEME_CSS)
            .extension_css(DEFAULT_THEME)
            .build(),
        )
        .child(MainView::create().build(ctx))
        .build(ctx)
    })
    .run();
}
