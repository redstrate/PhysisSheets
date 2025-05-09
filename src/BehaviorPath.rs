#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BehaviorPath {
exd: EXD,
exh: EXH,
}
impl BehaviorPath {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BehaviorPath").unwrap();let exd = game_data.read_excel_sheet("BehaviorPath", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BehaviorPathRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BehaviorPathRow { columns: row.columns.clone() }
}
}
pub struct BehaviorPathRow {
columns: Vec<ColumnData>,
}
impl BehaviorPathRow {
pub fn Speed(&self) -> &ColumnData {
&self.columns[0]
}
pub fn IsTurnTransition(&self) -> &ColumnData {
&self.columns[1]
}
pub fn IsFadeOut(&self) -> &ColumnData {
&self.columns[2]
}
pub fn IsFadeIn(&self) -> &ColumnData {
&self.columns[3]
}
pub fn IsWalking(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
}
