#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CSBonusMissionType {
exd: EXD,
exh: EXH,
}
impl CSBonusMissionType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CSBonusMissionType").unwrap();let exd = game_data.read_excel_sheet("CSBonusMissionType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CSBonusMissionTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CSBonusMissionTypeRow { columns: row.columns.clone() }
}
}
pub struct CSBonusMissionTypeRow {
columns: Vec<ColumnData>,
}
impl CSBonusMissionTypeRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
