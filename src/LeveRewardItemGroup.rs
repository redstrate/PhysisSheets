#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct LeveRewardItemGroup {
exd: EXD,
exh: EXH,
}
impl LeveRewardItemGroup {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("LeveRewardItemGroup").unwrap();let exd = game_data.read_excel_sheet("LeveRewardItemGroup", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> LeveRewardItemGroupRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
LeveRewardItemGroupRow { columns: row.columns.clone() }
}
}
pub struct LeveRewardItemGroupRow {
columns: Vec<ColumnData>,
}
impl LeveRewardItemGroupRow {
pub fn Item(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Count(&self) -> &ColumnData {
&self.columns[1]
}
pub fn IsHQ(&self) -> &ColumnData {
&self.columns[2]
}
}
