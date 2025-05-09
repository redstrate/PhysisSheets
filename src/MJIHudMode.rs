#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIHudMode {
exd: EXD,
exh: EXH,
}
impl MJIHudMode {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIHudMode").unwrap();let exd = game_data.read_excel_sheet("MJIHudMode", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIHudModeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIHudModeRow { columns: row.columns.clone() }
}
}
pub struct MJIHudModeRow {
columns: Vec<ColumnData>,
}
impl MJIHudModeRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Title(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
}
