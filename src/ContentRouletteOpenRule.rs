#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ContentRouletteOpenRule {
exd: EXD,
exh: EXH,
}
impl ContentRouletteOpenRule {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ContentRouletteOpenRule").unwrap();let exd = game_data.read_excel_sheet("ContentRouletteOpenRule", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ContentRouletteOpenRuleRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ContentRouletteOpenRuleRow { columns: row.columns.clone() }
}
}
pub struct ContentRouletteOpenRuleRow {
columns: Vec<ColumnData>,
}
impl ContentRouletteOpenRuleRow {
pub fn Type(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
}
