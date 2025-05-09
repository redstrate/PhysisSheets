#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GeneralAction {
exd: EXD,
exh: EXH,
}
impl GeneralAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GeneralAction").unwrap();let exd = game_data.read_excel_sheet("GeneralAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GeneralActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GeneralActionRow { columns: row.columns.clone() }
}
}
pub struct GeneralActionRow {
columns: Vec<ColumnData>,
}
impl GeneralActionRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Action(&self) -> &ColumnData {
&self.columns[3]
}
pub fn UnlockLink(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Recast(&self) -> &ColumnData {
&self.columns[6]
}
pub fn UIPriority(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[9]
}
}
