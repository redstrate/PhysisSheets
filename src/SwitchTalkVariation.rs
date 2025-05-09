#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SwitchTalkVariation {
exd: EXD,
exh: EXH,
}
impl SwitchTalkVariation {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SwitchTalkVariation").unwrap();let exd = game_data.read_excel_sheet("SwitchTalkVariation", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SwitchTalkVariationRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SwitchTalkVariationRow { columns: row.columns.clone() }
}
}
pub struct SwitchTalkVariationRow {
columns: Vec<ColumnData>,
}
impl SwitchTalkVariationRow {
pub fn Quest0(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Quest1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn DefaultTalk(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[3]
}
}
