#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MJICropSeed {
exd: EXD,
exh: EXH,
}
impl MJICropSeed {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MJICropSeed").unwrap();let exd = game_data.read_excel_sheet("MJICropSeed", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MJICropSeedRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MJICropSeedRow { columns: row.columns.clone() }
}
}
pub struct MJICropSeedRow {
columns: Vec<ColumnData>,
}
impl MJICropSeedRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SGB(&self) -> &ColumnData {
&self.columns[2]
}
}
