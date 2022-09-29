use druid::piet::{TextLayout, TextLayoutBuilder};
use druid::{
    kurbo::Line, piet::Text, BoxConstraints, Color, Env, Event, EventCtx, FontFamily, LayoutCtx,
    LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget,
};

use crate::app_data::AppData;
use crate::math::bezier3::Bezier3;
use crate::math::vec2::Vec2;

pub struct ContainerWidget {
    canvas_rect: Rect,
    canvas_viewport_screen: Rect,
    canvas_viewport_curve: Rect,
    dragging: bool,
    drag_start: Point,
    drag_pos: Point,
    drag_object: Option<usize>,
}

impl ContainerWidget {
    pub fn new() -> Self {
        Self {
            canvas_rect: Rect::ZERO,
            canvas_viewport_screen: Rect::ZERO,
            canvas_viewport_curve: Rect::ZERO,
            dragging: false,
            drag_start: Point::ZERO,
            drag_pos: Point::ZERO,
            drag_object: None,
        }
    }

    fn map_range(
        num: f32,
        domain_min: f32,
        domain_max: f32,
        range_min: f32,
        range_max: f32,
    ) -> f32 {
        (num - domain_min) * (range_max - range_min) / (domain_max - domain_min) + range_min
    }

    pub fn map_curvespace_to_screenspace(&self, source_vec: &Vec2<f32>) -> Point {
        Point::new(
            ContainerWidget::map_range(
                source_vec.x,
                self.canvas_viewport_curve.x0 as f32,
                self.canvas_viewport_curve.x1 as f32,
                self.canvas_viewport_screen.x0 as f32,
                self.canvas_viewport_screen.x1 as f32,
            )
            .into(),
            ContainerWidget::map_range(
                source_vec.y,
                self.canvas_viewport_curve.y0 as f32,
                self.canvas_viewport_curve.y1 as f32,
                self.canvas_viewport_screen.y1 as f32,
                self.canvas_viewport_screen.y0 as f32,
            )
            .into(),
        )
    }

    pub fn map_screenspace_to_curvespace(&self, source_point: Point) -> Vec2<f32> {
        Vec2::new(
            ContainerWidget::map_range(
                source_point.x as f32,
                (self.canvas_rect.x0 - (self.canvas_rect.x0 - self.canvas_viewport_screen.x0))
                    as f32,
                (self.canvas_rect.x1 - (self.canvas_rect.x1 - self.canvas_viewport_screen.x1))
                    as f32,
                self.canvas_viewport_curve.x0 as f32,
                self.canvas_viewport_curve.x1 as f32,
            ),
            ContainerWidget::map_range(
                source_point.y as f32,
                (self.canvas_rect.y0 - (self.canvas_rect.y0 - self.canvas_viewport_screen.y0))
                    as f32,
                (self.canvas_rect.y1 - (self.canvas_rect.y1 - self.canvas_viewport_screen.y1))
                    as f32,
                self.canvas_viewport_curve.y1 as f32,
                self.canvas_viewport_curve.y0 as f32,
            ),
        )
    }

    pub fn drag_point(&mut self, curve: &mut Bezier3) {
        if let Some(i) = self.drag_object {
            curve.control_points[i] = self.map_screenspace_to_curvespace(self.drag_pos);
        }
    }
}

impl Widget<AppData> for ContainerWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        let mut repaint: bool = false;

        if let Event::MouseDown(m) = event {
            if m.button.is_left() {
                self.dragging = true;
                self.drag_start = m.pos;
                self.drag_pos = m.pos;

                // Retrieve the first curve's control points and
                // convert them to screen space
                let control_points = data.curves[0]
                    .control_points
                    .iter()
                    .map(|p| self.map_curvespace_to_screenspace(p));

                // Check if the mouse is near a control point
                // If it is, set that as the active point for the drag
                // TODO: this is hacky, use a z-stack?
                for p in control_points.enumerate() {
                    if p.1.distance(self.drag_pos) < 10. {
                        self.drag_object = Some(p.0);
                        break;
                    }
                }

                repaint = true;
            }
        } else if let Event::MouseMove(m) = event {
            if self.dragging {
                self.drag_pos = Point::new(m.pos.x, m.pos.y);
                self.drag_point(&mut data.curves[0]); // TODO: Handle multiple draggable curves

                repaint = true;
            }
        } else if let Event::MouseUp(m) = event {
            if m.button.is_left() {
                // Update drag position in case mouse movement is captured here
                self.drag_pos = Point::new(m.pos.x, m.pos.y);
                self.drag_point(&mut data.curves[0]);

                // Clear drag event
                self.dragging = false;
                self.drag_start = Point::ZERO;
                self.drag_pos = Point::ZERO;
                self.drag_object = None;

                repaint = true;
            }
        }

        // Request a repaint if something happened
        if repaint {
            ctx.request_paint();
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppData,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        // Retrieve context size
        let context_size = ctx.size();
        let canvas_rect = context_size.to_rect();

        // Fill plot background
        ctx.fill(canvas_rect, &druid::Color::WHITE);

        // Create axis limit labels
        let x_min_label = ctx
            .text()
            .new_text_layout(format!("{:.2}", data.viewport.x0))
            .font(FontFamily::MONOSPACE, 10.0)
            .text_color(Color::BLACK)
            .build()
            .unwrap();
        let x_max_label = ctx
            .text()
            .new_text_layout(format!("{:.2}", data.viewport.x1))
            .font(FontFamily::MONOSPACE, 10.0)
            .text_color(Color::BLACK)
            .alignment(druid::TextAlignment::End)
            .build()
            .unwrap();
        let y_min_label = ctx
            .text()
            .new_text_layout(format!("{:.2}", data.viewport.y0))
            .font(FontFamily::MONOSPACE, 10.0)
            .text_color(Color::BLACK)
            .build()
            .unwrap();
        let y_max_label = ctx
            .text()
            .new_text_layout(format!("{:.2}", data.viewport.y1))
            .font(FontFamily::MONOSPACE, 10.0)
            .text_color(Color::BLACK)
            .alignment(druid::TextAlignment::End)
            .build()
            .unwrap();

        // Create plot area based on axis limit label widths
        let canvas_viewport_screen = Rect::from_center_size(
            canvas_rect.center(),
            canvas_rect
                .inset(
                    -(y_min_label
                        .image_bounds()
                        .width()
                        .max(y_max_label.image_bounds().width() + 15.)),
                )
                .size(),
        );

        // Cache viewports
        self.canvas_rect = canvas_rect;
        self.canvas_viewport_screen = canvas_viewport_screen;
        self.canvas_viewport_curve = data.viewport;

        // Draw outline of plot area
        ctx.stroke(canvas_viewport_screen, &Color::BLACK, 1.);

        // Draw axis limit labels
        ctx.draw_text(
            &x_min_label,
            (canvas_viewport_screen.x0, canvas_viewport_screen.y1),
        );
        ctx.draw_text(
            &x_max_label,
            (
                canvas_viewport_screen.x1 - x_max_label.image_bounds().width(),
                canvas_viewport_screen.y1,
            ),
        );
        ctx.draw_text(
            &y_min_label,
            (
                canvas_viewport_screen.x0 - y_min_label.image_bounds().width() - 4.,
                canvas_viewport_screen.y1 - y_min_label.image_bounds().height() - 4.,
            ),
        );
        ctx.draw_text(
            &y_max_label,
            (
                canvas_viewport_screen.x0 - y_max_label.image_bounds().width() - 4.,
                canvas_viewport_screen.y0,
            ),
        );

        // Plot curves
        // TODO: Handle multiple curves
        for curve in &data.curves {
            // Retrieve curve samples
            let curve_points = curve.render();

            // Convert samples from curve space to screen space
            let mut screen_points = Vec::new();
            for p in &curve_points {
                screen_points.push(self.map_curvespace_to_screenspace(p));
            }

            // Plot curve by drawing lines between sample points
            for i in 0..screen_points.len() - 1 {
                ctx.stroke(
                    Line::new(screen_points[i], screen_points[i + 1]),
                    &Color::TEAL,
                    1.,
                );
            }

            // Draw sample points on top of curve
            for p in screen_points {
                ctx.fill(Rect::from_center_size(p, Size::new(2., 2.)), &Color::RED);
            }

            // Draw control quad edges
            for i in 0..curve.control_points.len() {
                let j = if i == curve.control_points.len() - 1 {
                    0
                } else {
                    i + 1
                };
                ctx.stroke(
                    Line::new(
                        self.map_curvespace_to_screenspace(&curve.control_points[i]),
                        self.map_curvespace_to_screenspace(&curve.control_points[j]),
                    ),
                    &Color::GRAY,
                    1.,
                );
            }

            // Draw control points
            for p in &curve.control_points {
                ctx.fill(
                    Rect::from_center_size(
                        self.map_curvespace_to_screenspace(p),
                        Size::new(5., 5.),
                    ),
                    &Color::RED,
                );
            }
        }
    }
}
