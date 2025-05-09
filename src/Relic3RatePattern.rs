#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct Relic3RatePattern {
exd: EXD,
exh: EXH,
}
impl Relic3RatePattern {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Relic3RatePattern").unwrap();let exd = game_data.read_excel_sheet("Relic3RatePattern", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> Relic3RatePatternRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
Relic3RatePatternRow { columns: row.columns.clone() }
}
}
pub struct Relic3RatePatternRow {
columns: Vec<ColumnData>,
}
impl Relic3RatePatternRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[2]
}
}
