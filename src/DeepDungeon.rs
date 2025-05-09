#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DeepDungeon {
exd: EXD,
exh: EXH,
}
impl DeepDungeon {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DeepDungeon").unwrap();let exd = game_data.read_excel_sheet("DeepDungeon", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DeepDungeonRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DeepDungeonRow { columns: row.columns.clone() }
}
}
pub struct DeepDungeonRow {
columns: Vec<ColumnData>,
}
impl DeepDungeonRow {
pub fn PomanderSlot(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MagiciteSlot(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ContentFinderConditionStart(&self) -> &ColumnData {
&self.columns[3]
}
pub fn AetherpoolArm(&self) -> &ColumnData {
&self.columns[4]
}
pub fn AetherpoolArmor(&self) -> &ColumnData {
&self.columns[5]
}
pub fn DeepDungeonType(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[7]
}
}
