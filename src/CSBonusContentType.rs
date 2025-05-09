#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CSBonusContentType {
exd: EXD,
exh: EXH,
}
impl CSBonusContentType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CSBonusContentType").unwrap();let exd = game_data.read_excel_sheet("CSBonusContentType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CSBonusContentTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CSBonusContentTypeRow { columns: row.columns.clone() }
}
}
pub struct CSBonusContentTypeRow {
columns: Vec<ColumnData>,
}
impl CSBonusContentTypeRow {
pub fn Dialogue(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Image(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[2]
}
pub fn UnlockQuest(&self) -> &ColumnData {
&self.columns[3]
}
pub fn UnlockLink(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ContentType(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[7]
}
}
