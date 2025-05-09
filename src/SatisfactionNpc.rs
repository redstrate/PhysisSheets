#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct SatisfactionNpc {
exd: EXD,
exh: EXH,
}
impl SatisfactionNpc {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("SatisfactionNpc").unwrap();let exd = game_data.read_excel_sheet("SatisfactionNpc", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> SatisfactionNpcRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
SatisfactionNpcRow { columns: row.columns.clone() }
}
}
pub struct SatisfactionNpcRow {
columns: Vec<ColumnData>,
}
impl SatisfactionNpcRow {
pub fn SatisfactionNpcParams(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RankParams(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Npc(&self) -> &ColumnData {
&self.columns[3]
}
pub fn QuestRequired(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[5]
}
pub fn LevelUnlock(&self) -> &ColumnData {
&self.columns[6]
}
pub fn DeliveriesPerWeek(&self) -> &ColumnData {
&self.columns[7]
}
pub fn GlamourIndex(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown20(&self) -> &ColumnData {
&self.columns[10]
}
}
