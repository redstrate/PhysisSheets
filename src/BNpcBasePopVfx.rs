#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct BNpcBasePopVfx {
exd: EXD,
exh: EXH,
}
impl BNpcBasePopVfx {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("BNpcBasePopVfx").unwrap();let exd = game_data.read_excel_sheet("BNpcBasePopVfx", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> BNpcBasePopVfxRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
BNpcBasePopVfxRow { columns: row.columns.clone() }
}
}
pub struct BNpcBasePopVfxRow {
columns: Vec<ColumnData>,
}
impl BNpcBasePopVfxRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
}
