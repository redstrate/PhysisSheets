#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct PvPActionSort {
exd: EXD,
exh: EXH,
}
impl PvPActionSort {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("PvPActionSort").unwrap();let exd = game_data.read_excel_sheet("PvPActionSort", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> PvPActionSortRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
PvPActionSortRow { columns: row.columns.clone() }
}
}
pub struct PvPActionSortRow {
columns: Vec<ColumnData>,
}
impl PvPActionSortRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Action(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ActionType(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[4]
}
}
