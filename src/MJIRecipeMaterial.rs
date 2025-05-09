#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJIRecipeMaterial {
exd: EXD,
exh: EXH,
}
impl MJIRecipeMaterial {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJIRecipeMaterial").unwrap();let exd = game_data.read_excel_sheet("MJIRecipeMaterial", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJIRecipeMaterialRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJIRecipeMaterialRow { columns: row.columns.clone() }
}
}
pub struct MJIRecipeMaterialRow {
columns: Vec<ColumnData>,
}
impl MJIRecipeMaterialRow {
pub fn ItemPouch(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
}
