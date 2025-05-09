#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::EXH, common::Language};
pub struct GcArmyExpeditionMemberBonus {
exd: EXD,
exh: EXH,
}
impl GcArmyExpeditionMemberBonus {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("GcArmyExpeditionMemberBonus").unwrap();let exd = game_data.read_excel_sheet("GcArmyExpeditionMemberBonus", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> GcArmyExpeditionMemberBonusRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
GcArmyExpeditionMemberBonusRow { columns: row.columns.clone() }
}
}
pub struct GcArmyExpeditionMemberBonusRow {
columns: Vec<ColumnData>,
}
impl GcArmyExpeditionMemberBonusRow {
pub fn Race(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ClassJob(&self) -> &ColumnData {
&self.columns[1]
}
}
