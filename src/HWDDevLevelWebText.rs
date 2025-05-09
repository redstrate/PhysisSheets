#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct HWDDevLevelWebText {
exd: EXD,
exh: EXH,
}
impl HWDDevLevelWebText {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("HWDDevLevelWebText").unwrap();let exd = game_data.read_excel_sheet("HWDDevLevelWebText", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> HWDDevLevelWebTextRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
HWDDevLevelWebTextRow { columns: row.columns.clone() }
}
}
pub struct HWDDevLevelWebTextRow {
columns: Vec<ColumnData>,
}
impl HWDDevLevelWebTextRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
}
