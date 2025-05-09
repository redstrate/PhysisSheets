#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DeepDungeonMagicStone {
exd: EXD,
exh: EXH,
}
impl DeepDungeonMagicStone {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DeepDungeonMagicStone").unwrap();let exd = game_data.read_excel_sheet("DeepDungeonMagicStone", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DeepDungeonMagicStoneRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DeepDungeonMagicStoneRow { columns: row.columns.clone() }
}
}
pub struct DeepDungeonMagicStoneRow {
columns: Vec<ColumnData>,
}
impl DeepDungeonMagicStoneRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Tooltip(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[4]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[5]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[10]
}
}
