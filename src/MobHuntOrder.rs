#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct MobHuntOrder {
exd: EXD,
exh: EXH,
}
impl MobHuntOrder {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("MobHuntOrder").unwrap();let exd = game_data.read_excel_sheet("MobHuntOrder", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> MobHuntOrderRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
MobHuntOrderRow { columns: row.columns.clone() }
}
}
pub struct MobHuntOrderRow {
columns: Vec<ColumnData>,
}
impl MobHuntOrderRow {
pub fn Target(&self) -> &ColumnData {
&self.columns[0]
}
pub fn NeededKills(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Rank(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MobHuntReward(&self) -> &ColumnData {
&self.columns[4]
}
}
