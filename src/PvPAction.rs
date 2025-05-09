#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PvPAction {
exd: EXD,
exh: EXH,
}
impl PvPAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PvPAction").unwrap();let exd = game_data.read_excel_sheet("PvPAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PvPActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PvPActionRow { columns: row.columns.clone() }
}
}
pub struct PvPActionRow {
columns: Vec<ColumnData>,
}
impl PvPActionRow {
pub fn Action(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[5]
}
pub fn GrandCompany(&self) -> &ColumnData {
&self.columns[6]
}
}
