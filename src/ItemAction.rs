#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct ItemAction {
exd: EXD,
exh: EXH,
}
impl ItemAction {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("ItemAction").unwrap();let exd = game_data.read_excel_sheet("ItemAction", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> ItemActionRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
ItemActionRow { columns: row.columns.clone() }
}
}
pub struct ItemActionRow {
columns: Vec<ColumnData>,
}
impl ItemActionRow {
pub fn Type(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Data(&self) -> &ColumnData {
&self.columns[1]
}
pub fn DataHQ(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CondLv(&self) -> &ColumnData {
&self.columns[3]
}
pub fn CondBattle(&self) -> &ColumnData {
&self.columns[4]
}
pub fn CondPVP(&self) -> &ColumnData {
&self.columns[5]
}
pub fn CondPVPOnly(&self) -> &ColumnData {
&self.columns[6]
}
}
