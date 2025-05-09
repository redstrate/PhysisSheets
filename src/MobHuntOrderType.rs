#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MobHuntOrderType {
exd: EXD,
exh: EXH,
}
impl MobHuntOrderType {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MobHuntOrderType").unwrap();let exd = game_data.read_excel_sheet("MobHuntOrderType", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MobHuntOrderTypeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MobHuntOrderTypeRow { columns: row.columns.clone() }
}
}
pub struct MobHuntOrderTypeRow {
columns: Vec<ColumnData>,
}
impl MobHuntOrderTypeRow {
pub fn Quest(&self) -> &ColumnData {
&self.columns[0]
}
pub fn EventItem(&self) -> &ColumnData {
&self.columns[1]
}
pub fn OrderStart(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[3]
}
pub fn OrderAmount(&self) -> &ColumnData {
&self.columns[4]
}
}
