use fyrox::{
    core::{pool::Handle, visitor::prelude::*},
    gui::{
        grid::{Column, GridBuilder, Row},
        message::MessageDirection,
        progress_bar::{ProgressBarBuilder, ProgressBarMessage},
        text::TextBuilder,
        widget::WidgetBuilder,
        widget::WidgetMessage,
        BuildContext, HorizontalAlignment, UiNode, UserInterface, VerticalAlignment,
    },
};

#[derive(Visit, Default, Debug)]
pub struct LoadingScreen {
    pub root: Handle<UiNode>,
    progress_bar: Handle<UiNode>,
}

impl LoadingScreen {
    pub fn new(ctx: &mut BuildContext) -> Self {
        let progress_bar;
        let root = GridBuilder::new(
            WidgetBuilder::new().with_visibility(false).with_child(
                GridBuilder::new(
                    WidgetBuilder::new()
                        .on_row(1)
                        .on_column(1)
                        .with_child({
                            progress_bar =
                                ProgressBarBuilder::new(WidgetBuilder::new().on_row(1)).build(ctx);
                            progress_bar
                        })
                        .with_child(
                            TextBuilder::new(WidgetBuilder::new().on_row(0))
                                .with_horizontal_text_alignment(HorizontalAlignment::Center)
                                .with_vertical_text_alignment(VerticalAlignment::Center)
                                .with_text("Loading... Please wait.")
                                .build(ctx),
                        ),
                )
                .add_row(Row::stretch())
                .add_row(Row::strict(32.0))
                .add_column(Column::stretch())
                .build(ctx),
            ),
        )
        .add_column(Column::stretch())
        .add_column(Column::strict(400.0))
        .add_column(Column::stretch())
        .add_row(Row::stretch())
        .add_row(Row::strict(100.0))
        .add_row(Row::stretch())
        .build(ctx);
        Self { root, progress_bar }
    }

    pub fn resize(&self, ui: &UserInterface, width: f32, height: f32) {
        ui.send_message(WidgetMessage::width(
            self.root,
            MessageDirection::ToWidget,
            width,
        ));
        ui.send_message(WidgetMessage::height(
            self.root,
            MessageDirection::ToWidget,
            height,
        ));
    }

    pub fn set_progress(&self, ui: &UserInterface, progress: f32) {
        ui.send_message(ProgressBarMessage::progress(
            self.progress_bar,
            MessageDirection::ToWidget,
            progress,
        ));
    }
}
