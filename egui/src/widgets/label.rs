use crate::{
    widget_text::{WidgetTextGalley, WidgetTextJob},
    *,
};

/// Static text.
///
/// ```
/// # let ui = &mut egui::Ui::__test();
/// ui.label("Equivalent");
/// ui.add(egui::Label::new("Equivalent"));
/// ui.add(egui::Label::new("With Options").text_color(egui::Color32::RED));
/// ```
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct Label {
    text: WidgetText,
    wrap: Option<bool>,
    sense: Sense,
}

impl Label {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            wrap: None,
            sense: Sense::focusable_noninteractive(),
        }
    }

    pub fn text(&self) -> &str {
        self.text.text()
    }

    /// If `true`, the text will wrap to stay within the max width of the `Ui`.
    ///
    /// By default [`Self::wrap`] will be true in vertical layouts
    /// and horizontal layouts with wrapping,
    /// and false on non-wrapping horizontal layouts.
    ///
    /// Note that any `\n` in the text will always produce a new line.
    #[inline]
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = Some(wrap);
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).text_style(…))"]
    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text = self.text.text_style(text_style);
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).heading())"]
    pub fn heading(mut self) -> Self {
        self.text = self.text.heading();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).monospace())"]
    pub fn monospace(mut self) -> Self {
        self.text = self.text.monospace();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).code())"]
    pub fn code(mut self) -> Self {
        self.text = self.text.code();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).strong())"]
    pub fn strong(mut self) -> Self {
        self.text = self.text.strong();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).weak())"]
    pub fn weak(mut self) -> Self {
        self.text = self.text.weak();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).underline())"]
    pub fn underline(mut self) -> Self {
        self.text = self.text.underline();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).strikethrough())"]
    pub fn strikethrough(mut self) -> Self {
        self.text = self.text.strikethrough();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).italics())"]
    pub fn italics(mut self) -> Self {
        self.text = self.text.italics();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).small())"]
    pub fn small(mut self) -> Self {
        self.text = self.text.small();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).small_raised())"]
    pub fn small_raised(mut self) -> Self {
        self.text = self.text.small_raised();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).raised())"]
    pub fn raised(mut self) -> Self {
        self.text = self.text.raised();
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).background_color(…))"]
    pub fn background_color(mut self, background_color: impl Into<Color32>) -> Self {
        self.text = self.text.background_color(background_color);
        self
    }

    #[deprecated = "Replaced by Label::new(RichText::new(…).text_color())"]
    pub fn text_color(mut self, text_color: impl Into<Color32>) -> Self {
        self.text = self.text.color(text_color);
        self
    }

    /// Make the label respond to clicks and/or drags.
    ///
    /// By default, a label is inert and does not respond to click or drags.
    /// By calling this you can turn the label into a button of sorts.
    /// This will also give the label the hover-effect of a button, but without the frame.
    ///
    /// ``` rust
    /// # use egui::{Label, Sense};
    /// # let ui = &mut egui::Ui::__test();
    /// if ui.add(Label::new("click me").sense(Sense::click())).clicked() {
    ///     /* … */
    /// }
    /// ```
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }
}

impl Label {
    fn layout(self, ui: &Ui) -> WidgetTextJob {
        let available_width = ui.available_width();
        let (halign, justify) = if ui.is_grid() {
            (Align::LEFT, false) // TODO: remove special Grid hacks like these
        } else {
            (
                ui.layout().horizontal_placement(),
                ui.layout().horizontal_justify(),
            )
        };
        let mut text_job = self.layout_job(ui, 0.0, available_width);
        text_job.job.halign = halign;
        text_job.job.justify = justify;
        text_job
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_job(self, ui: &Ui, leading_space: f32, available_width: f32) -> WidgetTextJob {
        let mut text_job = self
            .text
            .layout_job(ui, self.wrap, available_width, TextStyle::Body);
        if let Some(first_section) = text_job.job.sections.first_mut() {
            first_section.leading_space = leading_space;
        }
        text_job
    }

    /// `has_focus`: the item is selected with the keyboard, so highlight with underline.
    /// `response_color`: Unless we have a special color set, use this.
    fn paint_galley(
        ui: &mut Ui,
        pos: Pos2,
        text_galley: WidgetTextGalley,
        has_focus: bool,
        response_color: Color32,
    ) {
        let underline = if has_focus {
            Stroke::new(1.0, response_color)
        } else {
            Stroke::none()
        };

        let override_text_color = if text_galley.galley_has_color {
            None
        } else {
            Some(response_color)
        };

        ui.painter().add(epaint::TextShape {
            pos,
            galley: text_galley.galley,
            override_text_color,
            underline,
            angle: 0.0,
        });
    }

    /// Do layout and place the galley in the ui, without painting it or adding widget info.
    pub(crate) fn layout_in_ui(self, ui: &mut Ui) -> (Pos2, WidgetTextGalley, Response) {
        let sense = self.sense;
        let max_width = ui.available_width();

        let should_wrap = self.wrap.unwrap_or_else(|| ui.wrap_text());

        if should_wrap
            && ui.layout().main_dir() == Direction::LeftToRight
            && ui.layout().main_wrap()
            && max_width.is_finite()
        {
            // On a wrapping horizontal layout we want text to start after the previous widget,
            // then continue on the line below! This will take some extra work:

            let cursor = ui.cursor();
            let first_row_indentation = max_width - ui.available_size_before_wrap().x;
            egui_assert!(first_row_indentation.is_finite());

            let mut text_job = self.layout_job(ui, first_row_indentation, max_width);
            text_job.job.first_row_min_height = cursor.height();
            text_job.job.halign = Align::Min;
            text_job.job.justify = false;
            let text_galley = text_job.layout(ui.fonts());

            let pos = pos2(ui.max_rect().left(), ui.cursor().top());
            assert!(
                !text_galley.galley.rows.is_empty(),
                "Galleys are never empty"
            );
            // collect a response from many rows:
            let rect = text_galley.galley.rows[0]
                .rect
                .translate(vec2(pos.x, pos.y));
            let mut response = ui.allocate_rect(rect, sense);
            for row in text_galley.galley.rows.iter().skip(1) {
                let rect = row.rect.translate(vec2(pos.x, pos.y));
                response |= ui.allocate_rect(rect, sense);
            }
            (pos, text_galley, response)
        } else {
            let text_galley = self.layout(ui).layout(ui.fonts());
            let (rect, response) = ui.allocate_exact_size(text_galley.size(), sense);
            let pos = match text_galley.galley.job.halign {
                Align::LEFT => rect.left_top(),
                Align::Center => rect.center_top(),
                Align::RIGHT => rect.right_top(),
            };
            (pos, text_galley, response)
        }
    }
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> Response {
        let (pos, galley, response) = self.layout_in_ui(ui);
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, galley.text()));
        let response_color = ui.style().interact(&response).text_color();
        Self::paint_galley(ui, pos, galley, response.has_focus(), response_color);
        response
    }
}
