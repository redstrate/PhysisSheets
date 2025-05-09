#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct DeliveryQuest {
exd: EXD,
exh: EXH,
}
impl DeliveryQuest {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("DeliveryQuest").unwrap();let exd = game_data.read_excel_sheet("DeliveryQuest", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> DeliveryQuestRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
DeliveryQuestRow { columns: row.columns.clone() }
}
}
pub struct DeliveryQuestRow {
columns: Vec<ColumnData>,
}
impl DeliveryQuestRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
}
