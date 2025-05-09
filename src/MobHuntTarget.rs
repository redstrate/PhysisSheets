#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MobHuntTarget {
exd: EXD,
exh: EXH,
}
impl MobHuntTarget {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MobHuntTarget").unwrap();let exd = game_data.read_excel_sheet("MobHuntTarget", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MobHuntTargetRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MobHuntTargetRow { columns: row.columns.clone() }
}
}
pub struct MobHuntTargetRow {
columns: Vec<ColumnData>,
}
impl MobHuntTargetRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[1]
}
pub fn FATE(&self) -> &ColumnData {
&self.columns[2]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[4]
}
}
