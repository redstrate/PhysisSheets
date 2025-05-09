#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BGMFade {
exd: EXD,
exh: EXH,
}
impl BGMFade {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BGMFade").unwrap();let exd = game_data.read_excel_sheet("BGMFade", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BGMFadeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BGMFadeRow { columns: row.columns.clone() }
}
}
pub struct BGMFadeRow {
columns: Vec<ColumnData>,
}
impl BGMFadeRow {
pub fn SceneOut(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SceneIn(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BGMFadeType(&self) -> &ColumnData {
&self.columns[2]
}
}
