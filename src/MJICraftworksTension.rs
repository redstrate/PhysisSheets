#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJICraftworksTension {
exd: EXD,
exh: EXH,
}
impl MJICraftworksTension {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJICraftworksTension").unwrap();let exd = game_data.read_excel_sheet("MJICraftworksTension", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJICraftworksTensionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJICraftworksTensionRow { columns: row.columns.clone() }
}
}
pub struct MJICraftworksTensionRow {
columns: Vec<ColumnData>,
}
impl MJICraftworksTensionRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
