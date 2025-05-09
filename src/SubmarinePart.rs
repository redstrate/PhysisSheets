#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SubmarinePart {
exd: EXD,
exh: EXH,
}
impl SubmarinePart {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SubmarinePart").unwrap();let exd = game_data.read_excel_sheet("SubmarinePart", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SubmarinePartRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SubmarinePartRow { columns: row.columns.clone() }
}
}
pub struct SubmarinePartRow {
columns: Vec<ColumnData>,
}
impl SubmarinePartRow {
pub fn Class(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Surveillance(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Retrieval(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Speed(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Range(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Favor(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Slot(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Rank(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Components(&self) -> &ColumnData {
&self.columns[8]
}
pub fn RepairMaterials(&self) -> &ColumnData {
&self.columns[9]
}
}
