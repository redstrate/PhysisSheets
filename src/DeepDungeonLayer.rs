#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DeepDungeonLayer {
exd: EXD,
exh: EXH,
}
impl DeepDungeonLayer {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DeepDungeonLayer").unwrap();let exd = game_data.read_excel_sheet("DeepDungeonLayer", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DeepDungeonLayerRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DeepDungeonLayerRow { columns: row.columns.clone() }
}
}
pub struct DeepDungeonLayerRow {
columns: Vec<ColumnData>,
}
impl DeepDungeonLayerRow {
pub fn RoomA(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RoomB(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RoomC(&self) -> &ColumnData {
&self.columns[2]
}
pub fn DeepDungeon(&self) -> &ColumnData {
&self.columns[3]
}
pub fn FloorSet(&self) -> &ColumnData {
&self.columns[4]
}
pub fn WepMinLv(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ArmourMinLv(&self) -> &ColumnData {
&self.columns[6]
}
}
