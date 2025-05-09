#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DawnMemberUIParam {
exd: EXD,
exh: EXH,
}
impl DawnMemberUIParam {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DawnMemberUIParam").unwrap();let exd = game_data.read_excel_sheet("DawnMemberUIParam", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DawnMemberUIParamRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DawnMemberUIParamRow { columns: row.columns.clone() }
}
}
pub struct DawnMemberUIParamRow {
columns: Vec<ColumnData>,
}
impl DawnMemberUIParamRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ClassPlural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn VoiceLine(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ClassSingular(&self) -> &ColumnData {
&self.columns[3]
}
}
