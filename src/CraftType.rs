#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct CraftType {
exd: EXD,
exh: EXH,
}
impl CraftType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("CraftType").unwrap();let exd = game_data.read_excel_sheet("CraftType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> CraftTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
CraftTypeRow { columns: row.columns.clone() }
}
}
pub struct CraftTypeRow {
columns: Vec<ColumnData>,
}
impl CraftTypeRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MainPhysical(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SubPhysical(&self) -> &ColumnData {
&self.columns[2]
}
}
