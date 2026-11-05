// Hook helper: draw_title (rendered after health & hunger bars)
fn appleskin_hud(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    crate::game::mod_appleskin_glue::on_hud(game, batch);
}
