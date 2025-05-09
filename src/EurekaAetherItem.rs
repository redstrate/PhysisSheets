#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct EurekaAetherItem {
exd: EXD,
exh: EXH,
}
impl EurekaAetherItem {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("EurekaAetherItem").unwrap();let exd = game_data.read_excel_sheet("EurekaAetherItem", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> EurekaAetherItemRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
EurekaAetherItemRow { columns: row.columns.clone() }
}
}
pub struct EurekaAetherItemRow {
columns: Vec<ColumnData>,
}
impl EurekaAetherItemRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[4]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[8]
}
}
