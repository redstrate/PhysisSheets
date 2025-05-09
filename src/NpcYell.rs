#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct NpcYell {
exd: EXD,
exh: EXH,
}
impl NpcYell {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("NpcYell").unwrap();let exd = game_data.read_excel_sheet("NpcYell", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> NpcYellRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
NpcYellRow { columns: row.columns.clone() }
}
}
pub struct NpcYellRow {
columns: Vec<ColumnData>,
}
impl NpcYellRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BalloonTime(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
pub fn OutputType(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[9]
}
pub fn IsBalloonSlow(&self) -> &ColumnData {
&self.columns[10]
}
pub fn BattleTalkTime(&self) -> &ColumnData {
&self.columns[11]
}
}
