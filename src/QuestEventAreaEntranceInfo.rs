#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct QuestEventAreaEntranceInfo {
exd: EXD,
exh: EXH,
}
impl QuestEventAreaEntranceInfo {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("QuestEventAreaEntranceInfo").unwrap();let exd = game_data.read_excel_sheet("QuestEventAreaEntranceInfo", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> QuestEventAreaEntranceInfoRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
QuestEventAreaEntranceInfoRow { columns: row.columns.clone() }
}
}
pub struct QuestEventAreaEntranceInfoRow {
columns: Vec<ColumnData>,
}
impl QuestEventAreaEntranceInfoRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Location(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[2]
}
}
