#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct TraitRecast {
exd: EXD,
exh: EXH,
}
impl TraitRecast {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TraitRecast").unwrap();let exd = game_data.read_excel_sheet("TraitRecast", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TraitRecastRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
TraitRecastRow { columns: row.columns.clone() }
}
}
pub struct TraitRecastRow {
columns: Vec<ColumnData>,
}
impl TraitRecastRow {
pub fn Trait(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Action(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Timeds(&self) -> &ColumnData {
&self.columns[2]
}
}
