#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct FCActivity {
exd: EXD,
exh: EXH,
}
impl FCActivity {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("FCActivity").unwrap();let exd = game_data.read_excel_sheet("FCActivity", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> FCActivityRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
FCActivityRow { columns: row.columns.clone() }
}
}
pub struct FCActivityRow {
columns: Vec<ColumnData>,
}
impl FCActivityRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SelfKind(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TargetKind(&self) -> &ColumnData {
&self.columns[2]
}
pub fn NumParam(&self) -> &ColumnData {
&self.columns[3]
}
pub fn FCActivityCategory(&self) -> &ColumnData {
&self.columns[4]
}
pub fn IconType(&self) -> &ColumnData {
&self.columns[5]
}
}
