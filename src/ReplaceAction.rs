#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ReplaceAction {
exd: EXD,
exh: EXH,
}
impl ReplaceAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ReplaceAction").unwrap();let exd = game_data.read_excel_sheet("ReplaceAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ReplaceActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ReplaceActionRow { columns: row.columns.clone() }
}
}
pub struct ReplaceActionRow {
columns: Vec<ColumnData>,
}
impl ReplaceActionRow {
pub fn Action(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ReplaceActions(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Param1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Param2(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Param3(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Type1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Type2(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Type3(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ReplaceSettable(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[9]
}
}
